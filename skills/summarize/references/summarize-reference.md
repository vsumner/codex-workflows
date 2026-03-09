# Summarize reference

## CLI quick matrix

### Install
- `npx -y @steipete/summarize <input>`
- `npm i -g @steipete/summarize`
- `npm i @steipete/summarize-core` for library-only usage

### Core command shapes

- `summarize <input>`
- `summarize <url-or-file>
- `summarize -` reads stdin (text or binary)

### Common flags

- `summarize <input> [flags]`
- `--model <provider/model>`
- `--model auto`
- `--timeout <duration>`
- `--retries <count>`
- `--length short|medium|long|xl|xxl|<chars>`
- `--language, --lang <language>`
- `--max-output-tokens <count>`
- `--force-summary`
- `--cli [provider]`
- `--stream auto|on|off`
- `--plain`
- `--no-color`
- `--theme aurora|ember|moss|mono`
- `--format md|text`
- `--markdown-mode off|auto|llm|readability`
- `--preprocess off|auto|always`
- `--extract`
- `--slides`
- `--slides-ocr`
- `--slides-dir <dir>`
- `--slides-scene-threshold <0.1-1.0>`
- `--slides-max <count>`
- `--slides-min-duration <seconds>`
- `--json`
- `--verbose`
- `--metrics off|on|detailed`
- `--firecrawl auto|off|always`
- `--youtube auto`
- `--video-mode auto|transcript|understand`
- `--no-cache`
- `--no-media-cache`

## Extension + daemon commands

- `summarize daemon install --token <TOKEN>`
- `summarize daemon status`
- `summarize daemon uninstall`
- `summarize refresh-free`
- `summarize refresh-free --set-default`

## Config file

- Location: `~/.summarize/config.json`
- Typical keys: `model.id`, `model.mode`, `model.rules`, `models`, `env`, `ui.theme`, `cache.media`, `media.videoMode`, `slides.*`, `openai.useChatCompletions`
- `model: { "mode": "auto" }` enables auto model ordering
- `--model` has the highest precedence, followed by `SUMMARIZE_MODEL` and config

## Environment variables of interest

- `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, `XAI_API_KEY`, `NVIDIA_API_KEY`
- `OPENAI_USE_CHAT_COMPLETIONS=1`
- `GEMINI_API_KEY` (or `GOOGLE_GENERATIVE_AI_API_KEY`, `GOOGLE_API_KEY`)
- `OPENROUTER_API_KEY`
- `openrouter` models use `--model openrouter/<author>/<slug>`
- `OPENROUTER` free preset is managed with `summarize refresh-free`
- `FIRECRAWL_API_KEY`
- `YT_DLP_PATH`
- `SUMMARIZE_THEME`
- `SUMMARIZE_NO_TRUECOLOR`
- `SUMMARIZE_TRUECOLOR`
- `SUMMARIZE_NO_COLOR`

## Troubleshooting patterns

- Extension injection messages: check extension site access and tab reload.
- Daemon connectivity errors: run daemon status, then inspect `~/.summarize/logs/daemon.err.log`.
- Fast-failing media input: switch providers or run with `--video-mode transcript`.
- Missing markdown rendering in TTY: use `--json` or `--plain`.

## Notes

- Full README for the latest behavior: https://github.com/steipete/summarize
- Docs index and deeper behavior: https://github.com/steipete/summarize/blob/main/docs/README.md
