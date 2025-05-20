Changelog
All notable changes to the Desktop Assistant project will be documented in this file.
The format is based on Keep a Changelog, and this project adheres to Semantic Versioning.

[Unreleased]

---

## [0.2.0] - 2025-05-19 (Sprint 2)

### Added
- **AI Chatbox Integration (Task 1)**
  - Integrated Grok 3 API for chat responses in `/apps/assistant`.
  - Added system prompt for Vietnamese conversational style.
- **Voice Chat Functionality (Task 2)**
  - Implemented STT (Whisper via Python service) for voice input transcription.
  - Implemented TTS (VietTTS) for voice output of assistant responses.
  - Added microphone and speaker UI controls in `ChatBox.svelte`.
- **Enhanced UI/UX for Chat (Task 3)**
  - Implemented backend event emissions from Rust for chat processing stages (`TRANSCRIBING`, `PROCESSING_API`, `SYNTHESIZING_VOICE`).
  - `ChatBox.svelte` now listens to these events to update UI.
  - Added dynamic user and assistant message bubbles to display ongoing interactions.
  - Implemented a status notification area showing current processing stage messages (e.g., "Transcribing voice...").
  - Refactored voice and text input flows for a turn-by-turn chat experience.

### Changed
- **AI Chatbox (Task 1)**
  - Modified `reqwest` calls in Rust backend to use `rustls-tls` feature for successful HTTPS requests to Grok API.
- **Voice Chat UI (Task 2)**
  - Updated `ChatBox.svelte` to ensure voice output indicator button remains visible and clickable.
- **Chat Flow (Task 3)**
  - Reworked `ChatBox.svelte` logic to be event-driven based on backend processing stages.

### Fixed
- **AI Chatbox (Task 1)**
  - Resolved `reqwest` HTTPS request failures to Grok API.
- **Voice Chat UI (Task 2)**
  - Corrected UI issue where voice output indicator button would disappear.
- **Rust Backend (Task 3)**
  - Corrected `AppHandle::emit` / `emit_all` calls and trait imports for event emission in `lib.rs` and `chathandle/mod.rs`.
  - Ensured `serde::Serialize` is correctly imported for payload structs.

---

## [0.0.1] - 2025-04-28 (Pre-Sprint 1 / Setup)

### Added
- Initialized monorepo structure with Turborepo (`/apps/assistant`, `/packages/*`, `/memory-bank/*`).
- Created `.clinerules` for Cline coding style, testing, and documentation rules.
- Created `.clineignore` to exclude sensitive files (`.env`, `/logs/*`, `/models/*`).
- Set up Memory Bank (`projectbrief.md`, `techContext.md`, `activeContext.md`, `progress.md`) for Cline context.
- Added initial `README.md` with project overview, setup, and usage.
- Created `CHANGELOG.md` for tracking changes.

### Changed
- None.

### Fixed
- None.

### Notes
- Project is in pre-Sprint 1 phase (structure setup, 28/04/2025).
- Next steps: Implement Sprint 1 tasks (Tauri app, Vosk STT, VietTTS, Cline setup, initial tests).
