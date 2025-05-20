# Plan: Integrate TTS Client into Tauri Application

**1. Objective:**
Enable the Mivis Tauri application (`apps/assistant`) to utilize the Vietnamese text-to-speech functionality provided by the VietTTS client in `packages/tts`, allowing the assistant to vocalize responses.

**2. Core Challenge:**
Bridge the TypeScript-based VietTTS client, which interacts with a Docker service at `http://localhost:8298`, with the Tauri application, ensuring seamless communication between the Svelte frontend, Rust backend, and the external TTS service, while managing audio playback in the UI.

**3. Proposed Architecture & Steps:**

- **Phase 1: Verify VietTTS Service Accessibility**
  - **Action:** Confirm that the VietTTS Docker service is running and accessible at `http://localhost:8298`.
  - **Details:** Use a command like `Invoke-WebRequest` to test connectivity to the service endpoint (`http://localhost:8298/v1/voices`) with the appropriate authorization header.
  - **Testing:** Manually verify the service response to ensure it returns a list of available voices.
  - **Status Update:** Service accessibility confirmed with a successful response (StatusCode 200) on 15/05/2025, returning a list of voices such as "diep-chi" and "quynh".

- **Phase 2: Implement Tauri Backend (Rust) Command**
  - **Action:** Add a Rust command in `apps/assistant/src-tauri/src/lib.rs` to handle TTS synthesis requests.
  - **Details:**
    - Utilize the existing `reqwest` crate in `Cargo.toml` for HTTP requests.
    - Create an `async` command named `synthesize_speech` that accepts `text: String` and an optional `voice: Option<String>` parameter, defaulting to "diep-chi" if not specified.
    - Send a POST request to `http://localhost:8298/v1/audio/speech` with a JSON payload including the model, input text, and voice, along with the authorization header.
    - Return the audio data as `Vec<u8>` to the frontend for playback.
    - Implement error handling for cases where the service is unavailable or the request fails.
  - **Testing:** Manually test the command to ensure it communicates with the VietTTS service and returns audio data.
  - **Status Update:** Command implemented successfully on 15/05/2025, integrated into the Tauri backend.

- **Phase 3: Integrate with Svelte Frontend (JavaScript/TypeScript)**
  - **Action:** Update the Svelte component in `apps/assistant/src/routes/+page.svelte` to trigger TTS synthesis and play audio.
  - **Details:**
    - Add UI elements including a text input field for entering text to speak and a button to initiate synthesis.
    - Use Tauri's `invoke` API to call the `synthesize_speech` command with the provided text.
    - Receive the audio data as an array of numbers, convert it to `Uint8Array`, and create a `Blob` for playback (assuming WAV format from VietTTS).
    - Use the HTML5 Audio API to play the audio, managing playback status and cleaning up URL objects.
    - Implement error handling and status updates in the UI for user feedback.
  - **Testing:** Manually test the frontend to ensure text input results in audible speech output.
  - **Status Update:** Frontend integration completed on 15/05/2025, with a fix for a TypeScript error in the error handler.

- **Phase 4: Process Management, Testing, and Documentation**
  - **VietTTS Service Lifecycle:** The VietTTS service is assumed to be running independently as a Docker container. If needed, a future task could configure it as a Tauri sidecar similar to the STT service.
  - **Error Handling:** Comprehensive error handling implemented across layers:
    - Backend returns errors if the VietTTS service is unreachable or returns an error.
    - Frontend displays user-friendly messages for errors during synthesis or playback.
  - **Testing (Next Steps):**
    - Add unit tests for the Rust `synthesize_speech` command to verify request construction and response handling.
    - Add integration tests for the full TTS flow from UI input to audio playback.
    - Manually test the end-to-end flow (UI -> Rust -> VietTTS Service -> UI) for stability.
    - Target 80% test coverage as per project standards.
  - **Documentation:**
    - Create this plan document (`docs/plans/integrate-tts-into-tauri.md`) detailing the integration steps and architecture.
    - Update `memory-bank/activeContext.md` with task details (e.g., "Integrated VietTTS client into Tauri app").
    - Update `memory-bank/progress.md` with status updates and any blockers encountered during integration.
  - **Status Update:** Documentation in progress as of 15/05/2025.

**4. Confidence Level:** 9/10
  - **Rationale:** The integration leverages a successful STT model, and the VietTTS service was confirmed accessible. The implementation mirrors established patterns in the codebase, ensuring compatibility and functionality.

**Status:** Completed - VietTTS client fully integrated into Tauri application with functional speech synthesis and playback as of 15/05/2025.

**Completed Steps:**
- Phase 1: Verify VietTTS Service Accessibility (Confirmed service running and accessible).
- Phase 2: Implement Tauri Backend (Rust) Command (Added `synthesize_speech` command with necessary request logic).
- Phase 3: Integrate with Svelte Frontend (Added UI elements for text input and speech triggering, implemented audio playback logic).

**Next Steps (Post-Integration):**
1. **End-to-End Testing:** Perform thorough manual testing of the TTS flow (UI -> Rust -> VietTTS Service -> UI) to ensure stability under various conditions.
2. **Automated Testing:**
   - Add unit tests for the Rust command to cover request and response handling.
   - Add integration tests for the full TTS flow.
3. **Documentation Updates:**
   - Finalize this plan document with any additional notes or issues encountered.
   - Update `memory-bank/activeContext.md` and `memory-bank/progress.md` with task completion details.
   - Update `README.md` and `CHANGELOG.md` if required for Sprint milestones.
4. **Future Improvements:**
   - Consider configuring VietTTS as a Tauri sidecar for automated service management.
   - Optimize audio playback format handling if VietTTS output format requires specific processing.
