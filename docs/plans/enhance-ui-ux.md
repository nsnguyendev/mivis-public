# Plan: Enhance UI/UX (Task 3)

## 1. Introduction

This plan outlines the steps to implement Task 3: Enhance UI for User Experience. This involves two main components:
1.  A `UI processing handle` function to display processing stages in bubble of the chat box UI.
2.  Enhancements to the `UI processing handle` for streaming API text responses.

This plan adheres to the project's `.clinerules` and overall architecture.


## 2. UI Processing Handle Function

*   **Objective:** To provide users with clear visual feedback on the assistant's processing status and to display API responses in a more engaging, streaming manner.
To provide more human being conversation chat box (turn by turn chat) like Messenger of Facebook. The chat box will have user bubbles and assistant bubbles. The user bubbles will show the content/text from user (transcribed text or transcribing stage). The assistant bubbles will show the content/text from API response (Processing request or API response)
*   **Location:** Primarily within `apps/assistant/src/lib/components/ChatBox.svelte`, potentially with new child Svelte components for modularity if needed.

### 2.1. Stage Display

*   **UI Elements & States:**
    *   A dedicated status area within the chat interface.
    *   States (managed by a reactive variable, e.g., `currentProcessingStage`):
        1.  `IDLE`
        2.  `TRANSCRIBING` (Text: "Transcribing voice...")
        3.  `PROCESSING_API` (Text: "Processing request...")
        4.  `SYNTHESIZING_VOICE` (Text: "Synthesizing voice...")
    *   Each active state should display its text and a loading indicator.
    *   Important note: The user and assistant content bubble behavior in chat box should meet the behavior expectation below:
        1.  `IDLE`: 
        user content bubble: nothing happen
        assistant content bubble: nothing happen
        2. `TRANSCRIBING`:
        user content bubble: (Text: "Transcribing voice...") as the return output of backend 
        assistant content bubble: nothing happen
        3. `PROCESSING_API`: 
        user content bubble: transcribed text
        assistant content bubble: (Text: "Processing request...") as the return output of backend 
        4. `SYNTHESIZING_VOICE`:
        user content bubble: transcribed text
        assistant content bubble: API response
    
*   **Event Flow & State Management:**
    *   Carefully change the event flow without break the current behavior.
    *   Tauri Rust backend emits events: `processing_stage_update` with payload `{ stage: 'TRANSCRIBING' | 'PROCESSING_API' | 'SYNTHESIZING_VOICE' | 'IDLE', message?: string }`.
    *   `ChatBox.svelte` listens and updates `currentProcessingStage`. Update chatbox: content in user bubble, content in assistant bubble.

### 2.2. Streaming Text Simulation Display

*   **Mechanism:** API responses displayed character-by-character or word-by-word.
*   **Configurable Parameters (internal):**
    *   `startDisplayOnStage`: Default: `'TTS_PLAYBACK_START'`. Option: `'API_RESPONSE_RECEIVED'`.
    *   `initialDelayMs`: Default: `0`.
    *   `streamingSpeedCharsPerMs`: e.g., `0.05` (50 chars/sec, adjust for natural feel).
*   **Implementation Approach (Svelte):**
    *   Store full API text.
    *   Reactive `displayedText` variable.
    *   Use `setTimeout` for delay and incremental updates to `displayedText`.


## 5. Preliminary List of Affected Files/New Files
*   **Modified:**
    *   `apps/assistant/src-tauri/src/main.rs` (or relevant handlers)
    *   `apps/assistant/src-tauri/Cargo.toml` (add `uuid` `chrono`)
    *   `apps/assistant/src/lib/components/ChatBox.svelte`

## 6. Open Questions/Considerations

*   **Icons:** Use a generic spinner icon initially.
*   **Streaming Configuration:** Internal constants are fine for now.
*   **Internationalization:** Stage display texts will be hardcoded initially (Vietnamese/English).
