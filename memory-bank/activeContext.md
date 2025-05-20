# Active Context for Mivis Desktop Assistant

## Current Sprint: Sprint 2

### Completed Tasks as of 19/05/2025
- **Task 1: Integrate AI Chatbox with LLM API** in `/apps/assistant`:
  - Successfully resolved the issue with `reqwest` failing to send HTTPS requests to `https://api.x.ai/v1/chat/completions` by adding the `rustls-tls` feature in `Cargo.toml`.
  - Enhanced error logging in `mod.rs` to diagnose API call failures.
  - Added a system prompt to instruct the API to respond shortly and in Vietnamese conversational format in `apps/assistant/src-tauri/src/chathandle/mod.rs`.
  - User confirmed the chat box functionality is working.

- **Task 2: Integrate STT-TTS to Chat Box for Voice Chat**:
  - Developed voice chat component in `ChatBox.svelte` with UI interactions for voice input (microphone button) and output (speaker icon).
  - Implemented necessary Tauri commands in Rust for STT transcription and TTS audio playback.
  - Fixed UI issue with `voice-output-indicator` button disappearing when clicked by updating `ChatBox.svelte` to ensure the button remains visible and clickable.
  - User confirmed the task is completed.

- **Task 3: Enhance UI for User Experience (Workflow Timing & UI Processing)**: (Completed on 19/05/2025)
  - Implemented plan `docs\plans\enhance-ui-ux.md`.
  - Modified Rust backend (`lib.rs`, `chathandle/mod.rs`) to emit `processing_stage_update` events for `TRANSCRIBING`, `PROCESSING_API`, and `SYNTHESIZING_VOICE` stages.
  - Updated `ChatBox.svelte` to listen for these events and manage UI states accordingly.
  - Added new UI elements for displaying current user/assistant message bubbles during active processing.
  - Implemented a status notification area to show current processing stage messages (e.g., "Transcribing voice...", "Processing request...").
  - Refactored voice and text input flows to align with the new event-driven stage management for a turn-by-turn chat experience.

### Current Focus
- (Sprint 2 tasks completed, awaiting next tasks or sprint planning)

### Relevant Files
- `apps/assistant/src-tauri/Cargo.toml`
- `apps/assistant/src-tauri/src/chathandle/mod.rs`
- `apps/assistant/src-tauri/src/lib.rs`
- `apps/assistant/src/lib/components/ChatBox.svelte`
- `packages/stt/src/stt.py`, `packages/stt/src/stt_service.py`
- `packages/tts/src/vietTTSClient.ts`
- `docs/plans/enhance-ui-ux.md`
