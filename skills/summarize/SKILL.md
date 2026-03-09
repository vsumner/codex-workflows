---
name: summarize
description: This skill should be used when the user asks to "summarize this URL", "summarize a PDF", "summarize audio or video", "summarize YouTube content", "configure summarize defaults", or "troubleshoot summarize CLI". Operates Steipete Summarize for fast summaries of URLs and media.
---

# Summarize

## Quick use

Use this skill when someone asks for:
- Summaries of URLs, PDFs, images, transcripts, YouTube, or audio/video.
- One-command setup for Summarize CLI or Side Panel mode.
- Model and output length tuning (`--model`, `--length`, `--max-output-tokens`).
- Configuration guidance and common Summarize failure triage.

## Install and verify

1. `npm i -g @steipete/summarize` or Homebrew `brew install steipete/tap/summarize`.
2. Run `summarize "https://example.com"`.
3. Run `summarize <input> --help` to confirm current flags.

For extension mode:

1. Install and open the Side Panel/Sidebar extension.
2. Copy the token shown in the panel.
3. Run `summarize daemon install --token <TOKEN>`.
4. Keep daemon status healthy with `summarize daemon status`.

## Input patterns

- URLs: `summarize "https://example.com"`
- Local files: `summarize "/path/to/file.pdf"`
- Stdin: `cat file.txt | summarize -`
- YouTube: `summarize "https://youtu.be/<id>" --youtube auto`
- Podcasts: `summarize "https://feeds.example.com/podcast.xml"`
- Generic media: `summarize "/path/to/audio.mp3"`

For media-first workflows use `--video-mode transcript` when you need transcription before summarization.

## Output and model controls

- `--model auto` for automatic provider fallback.
- `--model <provider/model>` for explicit backend, for example `openai/gpt-5-mini`.
- `--length short|medium|long|xl|xxl|<chars>` to tune target size.
- `--force-summary` to summarize even short inputs.
- `--max-output-tokens <count>` to hard-cap LLM output.
- `--language, --lang <language>` to control output language.
- `--stream on|off|auto` to control streaming.
- `--json` for diagnostics and machine-readable output.
- `--extract` (URLs only) to print extracted content and exit.
- `--slides`, `--slides-ocr`, and `--slides-dir` for video-slide workflows.

## Common failures

- Extension warning "Receiving end does not exist": enable extension site access and reload the tab.
- Daemon not reachable: run `summarize daemon status`; check `~/.summarize/logs/daemon.err.log`.
- Media or file provider failures: try a different provider, especially for PDFs/images where xAI is limited.
- Input returned unchanged: use `--force-summary` to force model summarization.
- Need deeper diagnostics: add `--verbose`.

## Configuration

- Base config is `~/.summarize/config.json`.
- Prefer `env` over `apiKeys`.
- Environment variables always win over config.
- Media cache and CLI options are often set in config for repeated workflows.

For full command reference and env mapping, read `references/summarize-reference.md`.
