use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use chrono::DateTime;
use chrono::Duration;
use chrono::TimeZone;
use chrono::Utc;
use clap::Args;
use clap::Parser;
use clap::Subcommand;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use walkdir::WalkDir;

#[derive(Debug, Parser)]
#[command(
    name = "codex-threads",
    about = "Search and summarize local Codex thread history",
    version
)]
struct Cli {
    /// Emit machine-readable JSON. Text output is intentionally minimal until the CLI stabilizes.
    #[arg(long, global = true)]
    json: bool,

    /// Codex home directory. Defaults to $CODEX_HOME or ~/.codex.
    #[arg(long, global = true)]
    codex_home: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Report local Codex history/index health.
    Doctor,
    /// Rebuild the compact thread index.
    Sync,
    /// Query thread metadata.
    Threads(ThreadsCommand),
    /// Search user-message history.
    Messages(MessagesCommand),
    /// Read rollout JSONL events for a session id.
    Events(EventsCommand),
    /// Detect repeated workflow patterns.
    Patterns(PatternsCommand),
    /// Suggest skill candidates from recent thread history.
    SkillCandidates(SinceArgs),
}

#[derive(Debug, Args)]
struct ThreadsCommand {
    #[command(subcommand)]
    command: ThreadsSubcommand,
}

#[derive(Debug, Subcommand)]
enum ThreadsSubcommand {
    /// List recent threads.
    Recent(LimitSinceArgs),
    /// Resolve a fuzzy thread name to candidate session ids.
    Resolve {
        query: String,
        #[arg(long, default_value_t = 10)]
        limit: usize,
    },
    /// Read user messages from one session.
    Read {
        session_id: String,
        #[arg(long, default_value_t = 80)]
        limit: usize,
    },
}

#[derive(Debug, Args)]
struct MessagesCommand {
    #[command(subcommand)]
    command: MessagesSubcommand,
}

#[derive(Debug, Subcommand)]
enum MessagesSubcommand {
    /// Search user-message history.
    Search {
        query: String,
        #[arg(long, default_value = "14d")]
        since: String,
        #[arg(long, default_value_t = 20)]
        limit: usize,
        /// Include full matching message text. Defaults to bounded snippets.
        #[arg(long)]
        full: bool,
        /// Maximum characters to include in each snippet when --full is not set.
        #[arg(long, default_value_t = 320)]
        snippet_chars: usize,
    },
}

#[derive(Debug, Args)]
struct EventsCommand {
    #[command(subcommand)]
    command: EventsSubcommand,
}

#[derive(Debug, Subcommand)]
enum EventsSubcommand {
    /// Read rollout JSONL events for a session id.
    Read {
        session_id: String,
        #[arg(long, default_value_t = 50)]
        limit: usize,
    },
}

#[derive(Debug, Args)]
struct PatternsCommand {
    #[command(subcommand)]
    command: PatternsSubcommand,
}

#[derive(Debug, Subcommand)]
enum PatternsSubcommand {
    /// Summarize recent workflow verbs and repeated user prompts.
    Recent(SinceArgs),
}

#[derive(Debug, Args)]
struct LimitSinceArgs {
    #[arg(long, default_value = "3d")]
    since: String,
    #[arg(long, default_value_t = 50)]
    limit: usize,
}

#[derive(Debug, Args)]
struct SinceArgs {
    #[arg(long, default_value = "7d")]
    since: String,
}

#[derive(Debug, Deserialize)]
struct HistoryRecord {
    session_id: String,
    ts: i64,
    text: String,
}

#[derive(Debug, Deserialize)]
struct SessionIndexRecord {
    id: String,
    thread_name: String,
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ThreadIndex {
    generated_at: DateTime<Utc>,
    codex_home: PathBuf,
    threads: Vec<ThreadSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ThreadSummary {
    session_id: String,
    thread_name: Option<String>,
    updated_at: Option<DateTime<Utc>>,
    message_count: usize,
    message_count_is_partial: bool,
    first_message_at: Option<DateTime<Utc>>,
    last_message_at: Option<DateTime<Utc>>,
    preview: Vec<String>,
}

#[derive(Debug, Serialize)]
struct MessageHit {
    session_id: String,
    thread_name: Option<String>,
    ts: DateTime<Utc>,
    snippet: String,
    text_bytes: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let codex_home = codex_home(cli.codex_home)?;
    let output = match cli.command {
        Command::Doctor => doctor(&codex_home)?,
        Command::Sync => sync(&codex_home)?,
        Command::Threads(command) => match command.command {
            ThreadsSubcommand::Recent(args) => {
                recent_threads(&codex_home, &args.since, args.limit)?
            }
            ThreadsSubcommand::Resolve { query, limit } => {
                resolve_thread(&codex_home, &query, limit)?
            }
            ThreadsSubcommand::Read { session_id, limit } => {
                read_thread(&codex_home, &session_id, limit)?
            }
        },
        Command::Messages(command) => match command.command {
            MessagesSubcommand::Search {
                query,
                since,
                limit,
                full,
                snippet_chars,
            } => search_messages(&codex_home, &query, &since, limit, full, snippet_chars)?,
        },
        Command::Events(command) => match command.command {
            EventsSubcommand::Read { session_id, limit } => {
                read_events(&codex_home, &session_id, limit)?
            }
        },
        Command::Patterns(command) => match command.command {
            PatternsSubcommand::Recent(args) => recent_patterns(&codex_home, &args.since)?,
        },
        Command::SkillCandidates(args) => skill_candidates(&codex_home, &args.since)?,
    };

    let _json = cli.json;
    println!("{}", serde_json::to_string_pretty(&output)?);

    Ok(())
}

fn codex_home(explicit: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(path) = explicit {
        return Ok(path);
    }
    if let Ok(path) = std::env::var("CODEX_HOME") {
        return Ok(PathBuf::from(path));
    }
    let home = dirs::home_dir().context("could not determine home directory")?;
    Ok(home.join(".codex"))
}

fn cache_path() -> Result<PathBuf> {
    let base = dirs::data_local_dir()
        .or_else(dirs::data_dir)
        .context("could not determine local data directory")?;
    Ok(base
        .join("codex-workflows")
        .join("codex-threads")
        .join("index.json"))
}

fn doctor(codex_home: &Path) -> Result<serde_json::Value> {
    let history_path = history_path(codex_home);
    let session_index_path = session_index_path(codex_home);
    let cache_path = cache_path()?;

    Ok(json!({
        "codex_home": codex_home,
        "history": file_status(&history_path),
        "session_index": file_status(&session_index_path),
        "sessions_dir": dir_status(&codex_home.join("sessions")),
        "archived_sessions_dir": dir_status(&codex_home.join("archived_sessions")),
        "index": file_status(&cache_path),
    }))
}

fn sync(codex_home: &Path) -> Result<serde_json::Value> {
    let index = build_index(codex_home)?;
    let path = cache_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    std::fs::write(&path, serde_json::to_vec_pretty(&index)?)
        .with_context(|| format!("write {}", path.display()))?;

    Ok(json!({
        "index_path": path,
        "thread_count": index.threads.len(),
        "generated_at": index.generated_at,
    }))
}

fn recent_threads(codex_home: &Path, since: &str, limit: usize) -> Result<serde_json::Value> {
    let cutoff = parse_since(since)?;
    let mut threads = load_or_build_index(codex_home)?.threads;
    threads.retain(|thread| thread.updated_at.is_some_and(|ts| ts >= cutoff));
    threads.sort_by_key(|thread| std::cmp::Reverse(thread.updated_at));
    threads.truncate(limit);
    Ok(json!({ "threads": threads }))
}

fn resolve_thread(codex_home: &Path, query: &str, limit: usize) -> Result<serde_json::Value> {
    let query = query.to_lowercase();
    let mut scored = load_or_build_index(codex_home)?
        .threads
        .into_iter()
        .filter_map(|thread| {
            let name = thread.thread_name.clone().unwrap_or_default();
            let haystack = format!("{} {}", thread.session_id, name).to_lowercase();
            let score = fuzzy_score(&haystack, &query)?;
            Some((score, thread))
        })
        .collect::<Vec<_>>();
    scored.sort_by_key(|(score, thread)| {
        (
            std::cmp::Reverse(*score),
            std::cmp::Reverse(thread.updated_at),
        )
    });
    let matches = scored
        .into_iter()
        .take(limit)
        .map(|(score, thread)| json!({ "score": score, "thread": thread }))
        .collect::<Vec<_>>();
    Ok(json!({ "matches": matches }))
}

fn read_thread(codex_home: &Path, session_id: &str, limit: usize) -> Result<serde_json::Value> {
    let names = load_session_names(codex_home)?;
    let mut messages = read_history(codex_home)?
        .into_iter()
        .filter(|message| message.session_id == session_id)
        .collect::<Vec<_>>();

    if messages.is_empty()
        && let Some(path) = find_rollout(codex_home, session_id)
    {
        messages = read_rollout_user_messages(&path, session_id)?;
    }

    let messages = messages
        .into_iter()
        .take(limit)
        .map(|message| {
            json!({
                "session_id": message.session_id,
                "ts": ts_to_datetime(message.ts),
                "text": message.text,
            })
        })
        .collect::<Vec<_>>();

    Ok(json!({
        "session_id": session_id,
        "thread_name": names.get(session_id),
        "messages": messages,
    }))
}

fn search_messages(
    codex_home: &Path,
    query: &str,
    since: &str,
    limit: usize,
    full: bool,
    snippet_chars: usize,
) -> Result<serde_json::Value> {
    let cutoff = parse_since(since)?;
    let needle = query.to_lowercase();
    let names = load_session_names(codex_home)?;
    let mut hits = Vec::new();
    for message in read_history(codex_home)? {
        let ts = ts_to_datetime(message.ts);
        if ts < cutoff || !message.text.to_lowercase().contains(&needle) {
            continue;
        }
        hits.push(MessageHit {
            thread_name: names.get(&message.session_id).cloned(),
            session_id: message.session_id,
            ts,
            snippet: snippet_for_query(&message.text, query, snippet_chars),
            text_bytes: message.text.len(),
            text: full.then_some(message.text),
        });
    }
    hits.sort_by_key(|hit| std::cmp::Reverse(hit.ts));
    hits.truncate(limit);

    Ok(json!({ "hits": hits }))
}

fn read_events(codex_home: &Path, session_id: &str, limit: usize) -> Result<serde_json::Value> {
    let path = find_rollout(codex_home, session_id);
    let Some(path) = path else {
        return Ok(json!({
            "session_id": session_id,
            "events": [],
            "error": "no rollout JSONL file found under sessions or archived_sessions"
        }));
    };

    let file = File::open(&path).with_context(|| format!("open {}", path.display()))?;
    let mut events = Vec::new();
    for line in BufReader::new(file).lines().take(limit) {
        let line = line?;
        let event = serde_json::from_str::<serde_json::Value>(&line)
            .unwrap_or_else(|_| json!({ "raw": line }));
        events.push(event);
    }

    Ok(json!({
        "session_id": session_id,
        "path": path,
        "events": events,
    }))
}

fn recent_patterns(codex_home: &Path, since: &str) -> Result<serde_json::Value> {
    let cutoff = parse_since(since)?;
    let index = load_or_build_index(codex_home)?;
    let mut verbs: BTreeMap<String, usize> = BTreeMap::new();
    for thread in index.threads {
        if thread
            .updated_at
            .is_some_and(|updated_at| updated_at >= cutoff)
            && let Some(name) = thread.thread_name
            && let Some(verb) = name.split_whitespace().next()
        {
            *verbs.entry(verb.to_lowercase()).or_default() += 1;
        }
    }

    Ok(json!({
        "since": cutoff,
        "thread_name_verbs": sorted_counts(verbs),
        "signals": skill_signal_counts(codex_home, cutoff)?,
    }))
}

fn skill_candidates(codex_home: &Path, since: &str) -> Result<serde_json::Value> {
    let cutoff = parse_since(since)?;
    let signals = skill_signal_counts(codex_home, cutoff)?;
    let candidates = signals
        .into_iter()
        .filter(|signal| signal.count >= 2)
        .map(|signal| {
            json!({
                "name": signal.name,
                "count": signal.count,
                "rationale": signal.rationale,
                "suggested_surface": signal.suggested_surface,
            })
        })
        .collect::<Vec<_>>();

    Ok(json!({ "since": cutoff, "candidates": candidates }))
}

fn build_index(codex_home: &Path) -> Result<ThreadIndex> {
    let names = load_session_index_records(codex_home)?
        .into_iter()
        .map(|entry| {
            let updated_at = DateTime::parse_from_rfc3339(&entry.updated_at)
                .ok()
                .map(|dt| dt.with_timezone(&Utc));
            (entry.id, (entry.thread_name, updated_at))
        })
        .collect::<HashMap<_, _>>();
    let rollouts = rollout_paths(codex_home);

    let mut threads: HashMap<String, ThreadSummary> = HashMap::new();
    for message in read_history(codex_home)? {
        let ts = ts_to_datetime(message.ts);
        let entry = threads
            .entry(message.session_id.clone())
            .or_insert_with(|| ThreadSummary {
                session_id: message.session_id.clone(),
                thread_name: names.get(&message.session_id).map(|(name, _)| name.clone()),
                updated_at: names
                    .get(&message.session_id)
                    .and_then(|(_, updated_at)| *updated_at),
                message_count: 0,
                message_count_is_partial: false,
                first_message_at: Some(ts),
                last_message_at: Some(ts),
                preview: Vec::new(),
            });
        entry.message_count += 1;
        entry.first_message_at = entry.first_message_at.min(Some(ts));
        entry.last_message_at = entry.last_message_at.max(Some(ts));
        if entry.preview.len() < 3 {
            entry.preview.push(truncate(&message.text, 240));
        }
    }

    for (session_id, (thread_name, updated_at)) in names {
        if threads.contains_key(&session_id) {
            continue;
        }

        let mut summary = ThreadSummary {
            session_id: session_id.clone(),
            thread_name: Some(thread_name),
            updated_at,
            message_count: 0,
            message_count_is_partial: false,
            first_message_at: None,
            last_message_at: None,
            preview: Vec::new(),
        };

        if let Some(path) = rollouts.get(&session_id) {
            for message in read_rollout_user_messages_limited(path, &session_id, Some(3))? {
                apply_message_to_summary(&mut summary, &message);
            }
            summary.message_count_is_partial = summary.message_count == 3;
        }

        threads.insert(session_id, summary);
    }

    let mut threads = threads.into_values().collect::<Vec<_>>();
    threads.sort_by_key(|thread| std::cmp::Reverse(thread.updated_at));

    Ok(ThreadIndex {
        generated_at: Utc::now(),
        codex_home: codex_home.to_path_buf(),
        threads,
    })
}

fn load_or_build_index(codex_home: &Path) -> Result<ThreadIndex> {
    let path = cache_path()?;
    if path.exists() {
        let index = serde_json::from_slice::<ThreadIndex>(&std::fs::read(&path)?)
            .with_context(|| format!("read {}", path.display()))?;
        if index.codex_home == codex_home {
            return Ok(index);
        }
    }
    build_index(codex_home)
}

fn load_session_names(codex_home: &Path) -> Result<HashMap<String, String>> {
    Ok(load_session_index_records(codex_home)?
        .into_iter()
        .map(|entry| (entry.id, entry.thread_name))
        .collect())
}

fn load_session_index_records(codex_home: &Path) -> Result<Vec<SessionIndexRecord>> {
    read_jsonl(&session_index_path(codex_home))
}

fn read_history(codex_home: &Path) -> Result<Vec<HistoryRecord>> {
    read_jsonl(&history_path(codex_home))
}

fn read_jsonl<T>(path: &Path) -> Result<Vec<T>>
where
    T: for<'de> Deserialize<'de>,
{
    let file = File::open(path).with_context(|| format!("open {}", path.display()))?;
    let mut records = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        if let Ok(record) = serde_json::from_str::<T>(&line) {
            records.push(record);
        }
    }
    Ok(records)
}

fn read_rollout_user_messages(path: &Path, session_id: &str) -> Result<Vec<HistoryRecord>> {
    read_rollout_user_messages_limited(path, session_id, None)
}

fn read_rollout_user_messages_limited(
    path: &Path,
    session_id: &str,
    limit: Option<usize>,
) -> Result<Vec<HistoryRecord>> {
    let file = File::open(path).with_context(|| format!("open {}", path.display()))?;
    let mut messages = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line?;
        let Ok(event) = serde_json::from_str::<serde_json::Value>(&line) else {
            continue;
        };
        if let Some(message) = user_message_from_event(&event, session_id) {
            messages.push(message);
            if limit.is_some_and(|limit| messages.len() >= limit) {
                break;
            }
        }
    }
    Ok(messages)
}

fn user_message_from_event(event: &serde_json::Value, session_id: &str) -> Option<HistoryRecord> {
    if event.get("type")?.as_str()? != "event_msg" {
        return None;
    }

    let payload = event.get("payload")?;
    if payload.get("type")?.as_str()? != "user_message" {
        return None;
    }

    let text = payload.get("message")?.as_str()?.trim();
    if text.is_empty() {
        return None;
    }

    Some(HistoryRecord {
        session_id: session_id.to_owned(),
        ts: event_timestamp(event)?,
        text: text.to_owned(),
    })
}

fn event_timestamp(event: &serde_json::Value) -> Option<i64> {
    let timestamp = event.get("timestamp")?.as_str()?;
    DateTime::parse_from_rfc3339(timestamp)
        .ok()
        .map(|dt| dt.timestamp())
}

fn apply_message_to_summary(summary: &mut ThreadSummary, message: &HistoryRecord) {
    let ts = ts_to_datetime(message.ts);
    summary.message_count += 1;
    summary.first_message_at = summary.first_message_at.min(Some(ts)).or(Some(ts));
    summary.last_message_at = summary.last_message_at.max(Some(ts)).or(Some(ts));
    if summary.updated_at.is_none_or(|updated_at| updated_at < ts) {
        summary.updated_at = Some(ts);
    }
    if summary.preview.len() < 3 {
        summary.preview.push(truncate(&message.text, 240));
    }
}

#[derive(Debug, Serialize)]
struct SkillSignal {
    name: &'static str,
    count: usize,
    rationale: &'static str,
    suggested_surface: &'static str,
}

fn skill_signal_counts(codex_home: &Path, cutoff: DateTime<Utc>) -> Result<Vec<SkillSignal>> {
    let mut signals = vec![
        SkillSignal {
            name: "review-fix-verify",
            count: 0,
            rationale: "Repeated review feedback and \"verify each finding\" loops.",
            suggested_surface: "skill plus optional GitHub helper CLI",
        },
        SkillSignal {
            name: "session-orientation",
            count: 0,
            rationale: "Repeated \"what's next\", branch, status, and PR-link orientation requests.",
            suggested_surface: "plugin skill over git/gh/codex-threads; CLI only if a missing primitive appears",
        },
        SkillSignal {
            name: "skill-routing",
            count: 0,
            rationale: "Repeated explicit requests to load language/domain skills.",
            suggested_surface: "skill routing guidance or thin dispatcher skill",
        },
        SkillSignal {
            name: "environment-debugging",
            count: 0,
            rationale: "Repeated local dependency, Storybook, Bun, toolchain, or CI failure diagnosis.",
            suggested_surface: "debug-environment skill",
        },
    ];

    for message in read_history(codex_home)? {
        let ts = ts_to_datetime(message.ts);
        if ts < cutoff {
            continue;
        }
        let text = message.text.to_lowercase();
        if text.contains("verify each finding") || text.contains("review") && text.contains("fix") {
            signals[0].count += 1;
        }
        if text.contains("whats next")
            || text.contains("what's next")
            || text.contains("what branch")
            || text.contains("uncommited changes")
            || text.contains("uncommitted changes")
            || text.contains("got links")
        {
            signals[1].count += 1;
        }
        if text.contains("load the") && text.contains("skill") {
            signals[2].count += 1;
        }
        if text.contains("storybook")
            || text.contains("bun ")
            || text.contains("failed to resolve")
            || text.contains("error:")
            || contains_word(&text, "ci")
        {
            signals[3].count += 1;
        }
    }

    Ok(signals)
}

#[derive(Debug, Serialize)]
struct Count {
    name: String,
    count: usize,
}

fn sorted_counts(counts: BTreeMap<String, usize>) -> Vec<Count> {
    let mut counts = counts
        .into_iter()
        .map(|(name, count)| Count { name, count })
        .collect::<Vec<_>>();
    counts.sort_by_key(|count| std::cmp::Reverse(count.count));
    counts
}

fn parse_since(since: &str) -> Result<DateTime<Utc>> {
    if let Some(days) = since.strip_suffix('d') {
        let days = days
            .parse::<i64>()
            .with_context(|| format!("parse {since}"))?;
        return Ok(Utc::now() - Duration::days(days));
    }
    if let Some(hours) = since.strip_suffix('h') {
        let hours = hours
            .parse::<i64>()
            .with_context(|| format!("parse {since}"))?;
        return Ok(Utc::now() - Duration::hours(hours));
    }
    Ok(DateTime::parse_from_rfc3339(since)
        .with_context(|| format!("parse {since} as RFC3339 timestamp"))?
        .with_timezone(&Utc))
}

fn ts_to_datetime(ts: i64) -> DateTime<Utc> {
    Utc.timestamp_opt(ts, 0)
        .single()
        .unwrap_or(DateTime::<Utc>::UNIX_EPOCH)
}

fn fuzzy_score(haystack: &str, query: &str) -> Option<usize> {
    if haystack.contains(query) {
        return Some(10_000 + query.len());
    }
    let mut score = 0;
    for token in query.split_whitespace() {
        if haystack.contains(token) {
            score += token.len();
        }
    }
    (score > 0).then_some(score)
}

fn find_rollout(codex_home: &Path, session_id: &str) -> Option<PathBuf> {
    let mut paths = rollout_paths(codex_home);
    if let Some(path) = paths.remove(session_id) {
        return Some(path);
    }

    for root in [
        codex_home.join("sessions"),
        codex_home.join("archived_sessions"),
    ] {
        if !root.exists() {
            continue;
        }
        for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file()
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.contains(session_id))
            {
                return Some(path.to_path_buf());
            }
        }
    }
    None
}

fn rollout_paths(codex_home: &Path) -> HashMap<String, PathBuf> {
    let mut paths = HashMap::new();
    for root in [
        codex_home.join("sessions"),
        codex_home.join("archived_sessions"),
    ] {
        if !root.exists() {
            continue;
        }
        for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
                continue;
            };
            if let Some(session_id) = session_id_from_rollout_filename(file_name) {
                paths.insert(session_id.to_owned(), path.to_path_buf());
            }
        }
    }
    paths
}

fn session_id_from_rollout_filename(file_name: &str) -> Option<&str> {
    let stem = file_name.strip_suffix(".jsonl")?;
    if !stem.starts_with("rollout-") {
        return Some(stem);
    }
    if stem.len() < 36 {
        return None;
    }
    let session_id = &stem[stem.len() - 36..];
    if session_id
        .chars()
        .all(|c| c.is_ascii_hexdigit() || c == '-')
    {
        Some(session_id)
    } else {
        None
    }
}

fn truncate(text: &str, max_chars: usize) -> String {
    let mut chars = text.chars();
    let truncated = chars.by_ref().take(max_chars).collect::<String>();
    if chars.next().is_some() {
        format!("{truncated}...")
    } else {
        truncated
    }
}

fn snippet_for_query(text: &str, query: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }

    let query = query.to_lowercase();
    let text_lower = text.to_lowercase();
    let Some(byte_start) = text_lower.find(&query) else {
        return truncate(text, max_chars);
    };

    let char_positions = text.char_indices().map(|(idx, _)| idx).collect::<Vec<_>>();
    let match_char_start = char_positions
        .iter()
        .position(|idx| *idx >= byte_start)
        .unwrap_or(char_positions.len());
    let context = max_chars.saturating_sub(query.chars().count()).min(80) / 2;
    let char_start = match_char_start.saturating_sub(context);

    let mut snippet = text
        .chars()
        .skip(char_start)
        .take(max_chars)
        .collect::<String>();
    if char_start > 0 {
        snippet = format!("...{snippet}");
    }
    if text.chars().count() > char_start + max_chars {
        snippet.push_str("...");
    }
    snippet
}

fn contains_word(text: &str, needle: &str) -> bool {
    text.split(|c: char| !c.is_alphanumeric())
        .any(|word| word == needle)
}

fn history_path(codex_home: &Path) -> PathBuf {
    codex_home.join("history.jsonl")
}

fn session_index_path(codex_home: &Path) -> PathBuf {
    codex_home.join("session_index.jsonl")
}

fn file_status(path: &Path) -> serde_json::Value {
    match std::fs::metadata(path) {
        Ok(metadata) => json!({
            "path": path,
            "exists": true,
            "bytes": metadata.len(),
            "modified": metadata.modified().ok().map(DateTime::<Utc>::from),
        }),
        Err(error) => json!({
            "path": path,
            "exists": false,
            "error": error.to_string(),
        }),
    }
}

fn dir_status(path: &Path) -> serde_json::Value {
    match std::fs::read_dir(path) {
        Ok(entries) => json!({
            "path": path,
            "exists": true,
            "entry_count_sample": entries.take(1_000).count(),
        }),
        Err(error) => json!({
            "path": path,
            "exists": false,
            "error": error.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;
    use std::io::Write;
    use std::time::SystemTime;

    #[test]
    fn snippets_center_the_query_and_avoid_full_text_dump() {
        let text = format!("{} needle {}", "a".repeat(500), "b".repeat(500));
        let snippet = snippet_for_query(&text, "needle", 80);

        assert!(snippet.contains("needle"));
        assert!(snippet.starts_with("..."));
        assert!(snippet.ends_with("..."));
        assert!(snippet.len() < text.len());
    }

    #[test]
    fn event_msg_user_messages_are_extracted_from_rollout_events() {
        let event = json!({
            "type": "event_msg",
            "timestamp": "2026-04-11T21:35:43.309Z",
            "payload": {
                "type": "user_message",
                "message": "whats next"
            }
        });

        let message = user_message_from_event(&event, "session-1").expect("message");

        assert_eq!(message.session_id, "session-1");
        assert_eq!(message.text, "whats next");
        assert_eq!(ts_to_datetime(message.ts).year(), 2026);
    }

    #[test]
    fn response_item_user_messages_are_ignored_to_avoid_duplicates() {
        let event = json!({
            "type": "response_item",
            "timestamp": "2026-04-11T21:35:43.309Z",
            "payload": {
                "type": "message",
                "role": "user",
                "content": [{"type": "input_text", "text": "whats next"}]
            }
        });

        assert!(user_message_from_event(&event, "session-1").is_none());
    }

    #[test]
    fn rollout_messages_can_backfill_session_index_only_threads() -> Result<()> {
        let codex_home = temp_codex_home()?;
        std::fs::create_dir_all(codex_home.join("sessions"))?;
        std::fs::write(codex_home.join("history.jsonl"), "")?;
        std::fs::write(
            codex_home.join("session_index.jsonl"),
            r#"{"id":"session-1","thread_name":"Review empty thread","updated_at":"2026-04-11T21:35:43.309Z"}"#,
        )?;

        let mut rollout = File::create(codex_home.join("sessions").join("session-1.jsonl"))?;
        writeln!(
            rollout,
            "{}",
            json!({
                "type": "event_msg",
                "timestamp": "2026-04-11T21:35:43.309Z",
                "payload": {
                    "type": "user_message",
                    "message": "Review recent sessions"
                }
            })
        )?;

        let index = build_index(&codex_home)?;
        let thread = index
            .threads
            .iter()
            .find(|thread| thread.session_id == "session-1")
            .expect("thread");

        assert_eq!(thread.message_count, 1);
        assert!(!thread.message_count_is_partial);
        assert_eq!(thread.preview, vec!["Review recent sessions"]);
        assert!(thread.first_message_at.is_some());
        assert!(thread.last_message_at.is_some());

        std::fs::remove_dir_all(codex_home)?;

        Ok(())
    }

    fn temp_codex_home() -> Result<PathBuf> {
        let nanos = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_nanos();
        let path =
            std::env::temp_dir().join(format!("codex-threads-test-{}-{nanos}", std::process::id()));
        std::fs::create_dir_all(&path)?;
        Ok(path)
    }
}
