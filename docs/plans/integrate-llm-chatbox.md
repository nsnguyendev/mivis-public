# Integration Plan for AI Chatbox with LLM 3 in Mivis Desktop Assistant

**Last Updated:** 17/05/2025

This document outlines the detailed plan for integrating the Vercel AI SDK with LLM API to implement an AI-powered chatbox in the Mivis Tauri app (`/apps/assistant`). The plan covers text-based chat functionality (Task 1 of Sprint 2) and prepares for voice chat integration (Task 2 of Sprint 2) using existing STT/TTS packages from Sprint 1. It adheres to the project's goals of security, modularity, and lightweight design (< 50MB RAM), following standards in `.clinerules`.

---

## Objective
Implement an AI-powered chatbox in the Mivis Tauri app with text and voice input/output capabilities, using Vercel AI SDK, LLM API, and existing STT/TTS packages. Ensure security, modularity, and readiness for voice chat (Task 2).

## Architectural Fit
- **Frontend**: Svelte component (`ChatBox.svelte`) for modular, lightweight UI (< 50MB RAM).
- **Backend**: Tauri Rust command in `/apps/assistant/src-tauri/src/lib.rs` to call LLM API securely via `reqwest`.
- **Shared Logic**: Reusable AI utilities in `/packages/ai` for API configurations and response parsing.
- **Security**: Keep `XAI_API_KEY` in Rust backend, use environment variables.
- **Task 2 Prep**: UI placeholders for voice controls, backend source parameter to distinguish text/voice inputs.
- **Clean Structure**: New functions stored in dedicated folders (e.g., `/src-tauri/src/chathandle`) for readability and maintainability.

## Detailed Plan

### Phase 1: Text-Based Chatbox with LLM

#### 1. Setup and Configuration
- **Workspace**: `apps/assistant`
- **Actions**:
  - Install Vercel AI SDK: `pnpm add ai@^3.4.0` in `/apps/assistant/package.json`.
  - Add `reqwest` for Rust backend in `/apps/assistant/src-tauri/Cargo.toml`:
    ```
    reqwest = { version = "0.12.15", features = ["json"] }
    dotenv = "0.15"
    ```
  - Use `XAI_API_KEY` from `.env`:
    ```
    XAI_API_KEY=your-key-here
    ```
  - Ensure `.env` is in `.gitignore`.
- **Error Mitigation**: Use pre-commit hook to scan for secrets. Test SDK initialization in isolation.
- **Task 2 Relevance**: Ensure API call structure supports voice transcriptions as text input.
- **Confidence**: 9/10

#### 2. Backend for Secure API Access
- **File**: `/src-tauri/src/chathandle/mod.rs`
- **Actions**:
  - Create Tauri command `invoke_llm_chat`:
- **Error Mitigation**: Retry 3 times (2s delay) for API failures. Log non-sensitive errors with `log::error!`. Return user-friendly error messages.
- **Task 2 Relevance**: `source` field allows distinguishing text vs. voice inputs for logging or processing.
- **Confidence**: 8/10 (API call needs testing)

#### 3. Svelte Chat Component UI
- **File**: `/apps/assistant/src/lib/components/ChatBox.svelte`
- **Actions**:
  - Create UI with:
    - Scrollable message area (user + AI).
    - Text input + send button.
    - Placeholder microphone button (disabled, for Task 2).
    - Placeholder speaker icon for TTS (Task 2).
  - Use Svelte stores:
    ```typescript
    import { writable } from 'svelte/store';
    interface Message {
        role: 'user' | 'assistant';
        content: string;
        source?: 'text' | 'voice';
    }
    const messages = writable<Message[]>([]);
    const isLoading = writable<boolean>(false);
    const error = writable<string | null>(null);
    ```
  - Style with Tailwind CSS, auto-scroll new messages with `scrollIntoView`.
  - Add ARIA labels for accessibility (e.g., “Chat input field”).
- **Error Mitigation**: Validate input (trim empty messages). Use async updates to prevent UI freeze. Show error notifications in UI.
- **Task 2 Relevance**: Microphone button and speaker icon reserved for STT/TTS. `source` in messages for voice input tracking.
- **Confidence**: 9/10

#### 4. Frontend Logic (Vercel AI SDK)
- **File**: `ChatBox.svelte`
- **Actions**:
  - Use manual logic instead of `useChat` (not ideal for Tauri):
    ```typescript
    import { invoke } from '@tauri-apps/api/tauri';
    async function sendMessage(input: string) {
        isLoading.set(true);
        error.set(null);
        messages.update(msgs => [...msgs, { role: 'user', content: input, source: 'text' }]);
        try {
            const response = await invoke('invoke_llm_chat', {
                messages: $messages
            });
            messages.update(msgs => [...msgs, { role: 'assistant', content: response }]);
        } catch (e) {
            error.set(`Failed to get response: ${e}`);
        }
        isLoading.set(false);
    }
    ```
  - Bind `sendMessage` to send button click.
- **Error Mitigation**: Timeout (10s) for `invoke`. Catch IPC errors, show “Connection issue, retrying…” in UI.
- **Task 2 Relevance**: `sendMessage` can accept voice transcriptions by setting `source: 'voice'`.
- **Confidence**: 8/10 (needs testing with Tauri IPC)

### Phase 2: Voice Chat Integration Prep

#### 5. STT/TTS Integration Prep
- **File**: `ChatBox.svelte`, `/apps/assistant/src-tauri/src/lib.rs`
- **Actions**:
  - Add UI placeholders:
    - Microphone button: `<button disabled>🎤</button>` (enable in Task 2 with STT command).
    - Speaker icon: `<span class="hidden">🔊</span>` (show in Task 2 for TTS).
  - Extend backend command for voice:
    ```rust
    // In invoke_llm_chat, log source for debugging
    if let Some(source) = messages.last().and_then(|m| m.source.as_ref()) {
        log::info!("Input source: {}", source);
    }
    ```
  - Plan STT flow (Task 2):
    - Use Web Audio API to capture audio, send to existing STT command (`invoke_stt_transcription`).
    - Set transcribed text as input, call `sendMessage` with `source: 'voice'`.
  - Plan TTS flow (Task 2):
    - On AI response, call existing TTS command (`/packages/tts`).
    - Add toggle for auto-play TTS in UI.
- **Error Mitigation**: Reserve UI space for “Recording…” and “Transcribing…” states. Plan timeout for STT recording (10s).
- **Confidence**: 8.5/10 (depends on STT/TTS commands)

### Phase 3: Testing and Documentation


#### 6. Documentation
- **Actions**:
  - Create `/docs/plans/integrate-llm-chatbox.md` with this plan.
  - Update Memory Bank:
    - `activeContext.md`: “Integrating AI chatbox with LLM, voice prep.”
    - `progress.md`: “Chatbox text done, voice in progress.”
    - `index-assistant-temp.md`:
      ```
      # Assistant App
      - **Feature**: AI Chatbox with LLM
      - **Description**: Text and voice chat using Vercel AI SDK and LLM API.
      - **Status**: In Progress
      ```
  - Update `techContext.md` if Tauri command proxy pattern is new.
  - Update `README.md`, `CHANGELOG.md` per `.clinerules`.
- **Error Mitigation**: Avoid sensitive data in docs. Commit docs separately.
- **Confidence**: 9/10

## Error Mitigation Strategies
- **API Key Security**: Use `dotenv` in Rust, pre-commit hook to block secrets.
- **Network Failures**: Exponential backoff (1s, 2s, 4s) for API calls, cache recent responses.
- **UI Responsiveness**: Limit messages to 20 in UI, use virtual scrolling.
- **Tauri Issues**: Test Vercel AI SDK with Tauri IPC, fallback to manual logic if needed.
- **Task 2 Risks**: Ensure UI layout supports voice controls, backend handles voice errors (e.g., “Transcription failed”).

## Visual Representation
```mermaid
graph TD
    A[User] -->|Text Input| B[Svelte ChatBox Component]
    A -->|Voice Input (Task 2)| C[STT Module - Whisper]
    C -->|Transcription| B
    B -->|Invoke via IPC| D[Tauri Backend - Rust Command]
    D -->|API Call via reqwest| E[LLM API]
    E -->|Response| D
    D -->|Return via IPC| B
    B -->|Display Response| A
    B -->|Audio Output (Task 2)| F[TTS Module - VietTTS]
    F -->|Synthesized Audio| A
    G[Environment Variables] -->|Secure API Key| D
    H[Error Handling] -->|Log Errors| I[Console - No Sensitive Data]
    H -->|User Feedback| B
```

## Notes
- **Streaming**: Use non-streaming for Task 1 (simpler). Plan streaming for Task 2 to support TTS (read chunks).
- **Dependencies**: Avoid `groq-sdk` (Node.js-only). Use `reqwest` for Rust, Vercel AI SDK for frontend logic.
- **Confidence**: 8.5/10 (needs testing for API and Task 2 prep).

---

This plan ensures a structured approach to integrating the AI chatbox with LLM 3, maintaining the Mivis project's standards for security, modularity, and performance while preparing for voice chat capabilities in Task 2 of Sprint 2.
