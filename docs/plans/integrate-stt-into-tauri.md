# Plan: Integrate STT Client into Tauri Application

**1. Objective:**
Enable the Mivis Tauri application (`apps/assistant`) to utilize the Vietnamese speech-to-text functionality provided by the `packages/stt` (which uses a Python script with Faster Whisper).

**2. Core Challenge:**
Bridge the Python-based STT script (which relies on a Conda environment and specific model paths) with the Tauri application, which is built with Rust for the backend and Svelte (JavaScript/TypeScript) for the frontend.

**3. Proposed Architecture & Steps:**

*   **Phase 1: Prepare the Python STT Script as a Local HTTP Service**
    *   **Action:** Create a new file `packages/stt/src/stt_service.py` (for the HTTP service) and potentially refine `packages/stt/src/stt.py` (for core transcription logic).
    *   **Details:**
        *   **Separation of Concerns:** The new `stt_service.py` will house the Flask/FastAPI application logic (endpoints, request/response handling). The existing `stt.py` can be refactored to focus solely on the core transcription function (`transcribe_audio`).
        *   **HTTP Endpoint:** Create an HTTP endpoint (e.g., `POST /transcribe`) using Flask or FastAPI that accepts an audio file.
        *   **Audio Resampling:** Implement logic to ensure the audio is at a 16kHz sample rate using `ffmpeg` and `soundfile`. Resample if necessary before passing to the transcription function.
        *   **Transcription & Response:** Call the `transcribe_audio` function and return the transcription as a JSON response.
        *   **Error Handling & Dependencies:** Implement robust error handling and add necessary dependencies (Flask/FastAPI, soundfile, etc.) to the Conda environment.
        *   Ensure the service can reliably access the `PhoWhisper-ct2-FasterWhisper` model.
    *   **Testing:** Test this local Python API independently.

*   **Phase 2: Implement Tauri Backend (Rust) Command**
    *   **Action:** Create/modify Rust code in `apps/assistant/src-tauri/`.
    *   **Details:**
        *   Add `reqwest` crate to `apps/assistant/src-tauri/Cargo.toml`.
        *   Define a new Tauri `async` command (e.g., `invoke_stt_transcription`) in `src-tauri/src/lib.rs`.
        *   This command accepts an `audio_file_path: String`, reads the file, creates a multipart form, sends a POST request to the Python STT service, parses the JSON response, and returns the transcription.
        *   Implement temporary file cleanup on the Rust side after sending the request.
        *   Register the new command in `src-tauri/src/lib.rs`.
    *   **Status Update:** Path resolution issues for launching the sidecar script from Rust (`app_handle.path().resolve()`) have been resolved. The sidecar launch mechanism is confirmed working.

*   **Phase 3: Integrate with Svelte Frontend (JavaScript/TypeScript)**
    *   **Action:** Modify Svelte components in `apps/assistant/src/routes/+page.svelte`.
    *   **Details:**
        *   Add UI elements for recording (Start/Stop button), status display, and transcription output.
        *   Use the Web Audio API (`MediaRecorder`) to capture audio.
        *   Convert the recorded audio `Blob` to WAV format (ideally at 16kHz) in JavaScript before saving. (Note: Current implementation saves as webm and relies on backend resampling - needs refinement).
        *   Save the audio data to a temporary file using Tauri's `fs` API (`@tauri-apps/api/fs`).
        *   Invoke the Rust command `invoke_stt_transcription` with the path to the temporary file.
        *   Display the transcription or error messages in the UI.

*   **Phase 4: Process Management, Testing, and Documentation**
    *   **STT Service Lifecycle:** Configure the Python STT service to run as a Tauri sidecar using a batch script (`packages/stt/run_stt_service.bat`) and `tauri.conf.json`. The launch from the Rust `.setup()` hook is confirmed working.
    *   **Error Handling:** Implement comprehensive error handling across all layers.
    *   **Testing (Next Steps):**
        *   Perform end-to-end manual testing of the STT flow (UI -> Rust -> Python -> UI).
        *   Add unit tests for the Python service (including resampling).
        *   Add integration tests for the full flow.
    *   **Documentation:**
        *   Update this plan document (`docs/plans/integrate-stt-into-tauri.md`).
        *   Update `apps/assistant/index-assistant-temp.md` to reflect the new STT integration.
        *   Update `memory-bank/activeContext.md` (task details) and `memory-bank/progress.md` (status, blockers).

**4. Confidence Level:** 9/10 (Updated after successful integration)
    *   **Resolved Challenges:** Ensured correct audio format conversion/handling in Python by forcing WAV conversion with `ffmpeg`, resolved endpoint invocation issues, and maintained robust path handling for models/temp files.

**Status:** Completed - Python STT service fully integrated into Tauri application with successful transcription.

**Completed Steps:**
- Phase 1: Prepare the Python STT Script as a Local HTTP Service (Implemented with resampling and transcription logic, forced WAV conversion for format compatibility).
- Phase 2: Implement Tauri Backend (Rust) Command (Added `reqwest`, created command, logic for passing audio data directly to Rust, temporary file handling in Rust).
- Phase 3: Integrate with Svelte Frontend (JavaScript/TypeScript) (Added UI elements, recording logic, passing audio data to Rust).
- Phase 4: Process Management (Configured Python service as a Tauri sidecar. Sidecar launch mechanism using `run_stt_service.bat` with UAC elevation, `build.rs` for copying the script, and `conda.bat run` for execution is fully operational. The Python STT service starts and processes requests successfully.)

**Resolution of Previous Blocker (as of 2025-05-15):**
The initial blocker with the Python STT service not being reached was resolved by fixing a `datetime` import issue. The subsequent audio format recognition error (`"Error opening 'D:\\_project\\mivis\\packages\\stt\\src\\temp_audio\\audio_XXXXX_XXXXXXX.wav': Format not recognised."`) was addressed by updating `resample_audio` to force conversion to WAV format using `ffmpeg`, ensuring compatibility with the `soundfile` library used for processing.

**Next Steps (Post-Integration):**
1. **End-to-End Testing:** Perform thorough end-to-end manual testing of the STT flow (UI -> Rust -> Python -> UI) to ensure stability under various conditions.
2. **Automated Testing:**
    *   Add unit tests for the Python service, specifically covering audio format conversion and resampling.
    *   Add integration tests for the full STT flow.
3. **Documentation Updates:**
    *   Update `apps/assistant/index-assistant-temp.md` to reflect the completed STT integration.
    *   Update `memory-bank/activeContext.md` and `memory-bank/progress.md` with task completion details.
    *   Update `README.md` and `CHANGELOG.md` if required for Sprint milestones.
4. **Future Improvements:**
    *   Refine frontend audio handling to potentially convert to WAV format (ideally at 16kHz) before sending to the backend, reducing backend processing load.
