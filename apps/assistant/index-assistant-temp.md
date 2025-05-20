ADR: Assistant Index Documentation



Introduction
Prologue

Updated .\apps\assistant\index-assistant-temp.md to document the purpose, usage package in Mivis projects.




Discussion

The folder handles frontend of this Mivis project using Tauri with Svelte-Kit, and its documentation in index-assistant-temp.md needs to clearly explain its role and usage for using VS Code with Cline. Consistent, concise this files ensure easy navigation and integration with Cline for project management.




Solution
Purpose & Usage:

Purpose: Summarizes the folder, which provides the Tauri application frontend using Svelte-Kit. It now includes integration with the STT (Speech-to-Text) service.

# use tauri for project:

```bash
rimraf apps/assistant && pnpm create tauri-app@latest -- --template svelte
```

# sample usage

```bash
turbo run dev
```

# STT Integration:
The application now includes basic UI and logic in `src/routes/+page.svelte` to record audio, save it temporarily, and send it to the local Python STT service via a Rust Tauri command (`invoke_stt_transcription` in `src-tauri/src/lib.rs`). The Python service (`packages/stt/src/stt_service.py`) handles resampling and transcription. The Python service is configured to run as a Tauri sidecar via `src-tauri/tauri.conf.json` and `packages/stt/run_stt_service.bat`.

# AI Chatbox Integration:
The application now includes a Svelte component `ChatBox.svelte` in `src/lib/components/` for AI chat functionality, integrated into the main page `src/routes/+page.svelte`. The backend uses a Rust Tauri command (`invoke_grok_chat` in `src-tauri/src/chathandle/mod.rs`) to interact with the xAI Grok 3 API for natural language responses, with secure handling of API keys via environment variables loaded by `dotenv`.

Consequences

Pros: Updated index-assistant-temp.md provides clear guidance for using the template and the new STT integration. Links improve navigation across packages in VS Code via Cline.

Cons: Manual updates needed for changes in STT package or dependencies. Frontend WAV conversion is a pending TODO. The complexity of `ChatBox.svelte` has increased due to more detailed state management for UI feedback.

Status: File updated, compatible with Cline for development. STT, TTS, Grok API, and enhanced UI/UX for voice commands are integrated.
