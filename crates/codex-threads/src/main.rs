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
    first_message_at: Option<DateTime<Utc>>,
    last_message_at: Option<DateTime<Utc>>,
    preview: Vec<String>,
}

#[derive(Debug, Serialize)]
struct MessageHit {
    session_id: String,
    thread_name: Option<String>,
    ts: DateTime<Utc>,
    text: String,
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
            } => search_messages(&codex_home, &query, &since, limit)?,
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
    let messages = read_history(codex_home)?
        .into_iter()
        .filter(|message| message.session_id == session_id)
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
            text: message.text,
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
        threads.entry(session_id.clone()).or_insert(ThreadSummary {
            session_id,
            thread_name: Some(thread_name),
            updated_at,
            message_count: 0,
            first_message_at: None,
            last_message_at: None,
            preview: Vec::new(),
        });
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

fn truncate(text: &str, max_chars: usize) -> String {
    let mut chars = text.chars();
    let truncated = chars.by_ref().take(max_chars).collect::<String>();
    if chars.next().is_some() {
        format!("{truncated}...")
    } else {
        truncated
    }
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
