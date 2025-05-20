<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { appDataDir, join } from "@tauri-apps/api/path"; // Import join
  import { writeFile, create, BaseDirectory } from "@tauri-apps/plugin-fs"; // Import createDir
  import { onMount } from 'svelte';
  import ChatBox from '../lib/components/ChatBox.svelte';

  let name = $state("");
  let greetMsg = $state("");

  // STT State
  let isRecording = $state(false);
  let mediaRecorder: MediaRecorder | null = null;
  let audioChunks: BlobPart[] = [];
  let transcription = $state("");
  let sttError = $state("");
  let sttStatus = $state("Idle");
  let tauriApiReady = $state(false);

  // TTS State
  let textToSpeak = $state("");
  let ttsStatus = $state("Idle");
  let ttsError = $state("");
  let isPlaying = $state(false);

  onMount(() => {
    if (window.__TAURI_INTERNALS__) {
      tauriApiReady = true;
      console.log("Tauri API ready onMount.");
    } else {
      console.warn("Tauri API not immediately available onMount. Polling...");
      let attempts = 0;
      const interval = setInterval(() => {
        attempts++;
        if (window.__TAURI_INTERNALS__) {
          tauriApiReady = true;
          console.log(`Tauri API became ready after ${attempts} poll attempts.`);
          clearInterval(interval);
        } else if (attempts > 20) { // e.g., try for ~2 seconds
          console.error("Tauri API did not become ready after multiple poll attempts.");
          sttError = "Tauri API initialization failed. Please restart the application.";
          sttStatus = "Error: Tauri API failed to initialize.";
          clearInterval(interval);
        }
      }, 100);
    }
  });

  async function ensureTauriApiReady() {
    if (tauriApiReady) return true;
    // Check again if onMount's poller might have set it
    if (window.__TAURI_INTERNALS__) {
        tauriApiReady = true; // Update state if poller was slow or missed
        return true;
    }
    
    const errorMsg = "Tauri API is not ready. Cannot proceed.";
    sttError = errorMsg;
    sttStatus = "Error: Tauri API not ready.";
    ttsError = errorMsg;
    ttsStatus = "Error: Tauri API not ready.";
    console.error(errorMsg);
    return false;
  }

  async function startRecording() {
    sttError = "";
    transcription = "";
    
    if (!await ensureTauriApiReady()) {
        sttStatus = "Error: Tauri API not ready for recording.";
        return;
    }
    sttStatus = "Requesting microphone access...";
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      mediaRecorder = new MediaRecorder(stream);
      audioChunks = [];

      mediaRecorder.ondataavailable = (event) => {
        audioChunks.push(event.data);
      };

      mediaRecorder.onstop = async () => {
        sttStatus = "Processing audio...";
        if (!await ensureTauriApiReady()) return; // Check again before critical API calls

        const audioBlob = new Blob(audioChunks, { type: "audio/webm" }); // MediaRecorder typically outputs webm or ogg

        // TODO: Implement robust audio conversion to WAV (16kHz) here in the frontend.
        // This is a placeholder. A dedicated library or Web Audio API processing is needed.
        // For now, we'll proceed assuming the Python backend's resampling is sufficient,
        // but saving as WAV is preferred if possible in JS.
        console.warn("Frontend WAV conversion not fully implemented. Relying on backend resampling.");
        const wavBlob = audioBlob; // Placeholder: Assume audioBlob is or can be treated as WAV for now.
                                   // Replace this with actual conversion logic.

        const appDataDirPath = await appDataDir();
        // Use .wav extension as the backend expects WAV or will resample to WAV
        const tempFileName = `recording_${Date.now()}.wav`;
        // Correctly join the path
        const tempFilePath = await join(appDataDirPath, tempFileName); 

        sttStatus = `Saving temporary file: ${tempFilePath}`;
        try {
          // Convert Blob to ArrayBuffer
          const arrayBuffer = await wavBlob.arrayBuffer();
          const uint8Array = new Uint8Array(arrayBuffer);

          sttStatus = "Transcribing...";
          // Invoke the Rust command, passing the audio data directly
          const result = await invoke<string>("invoke_stt_transcription", { audioData: Array.from(uint8Array) }); // Pass as Array<number> for serialization

          transcription = result;
          sttStatus = "Transcription complete.";

          // No temporary file to clean up on the frontend side in this approach.

        } catch (error) {
          console.error("STT Process Error:", error);
          sttError = `STT Error: ${error}`;
          sttStatus = "Error during transcription.";
        } finally {
           // No temporary file to clean up on the frontend side in this approach.
        }

        // Stop the microphone stream tracks
        stream.getTracks().forEach(track => track.stop());
      };

      mediaRecorder.start();
      isRecording = true;
      sttStatus = "Recording...";

    } catch (err) {
      console.error("Microphone access error:", err);
      sttError = `Failed to get microphone access: ${err}`;
      sttStatus = "Microphone access denied or failed.";
    }
  }

  function stopRecording() {
    if (mediaRecorder && isRecording) {
      mediaRecorder.stop();
      isRecording = false;
      sttStatus = "Stopping recording...";
    }
  }

  async function startSpeaking() {
    ttsError = "";
    ttsStatus = "Initiating speech synthesis...";
    
    if (!await ensureTauriApiReady()) {
      ttsStatus = "Error: Tauri API not ready for speech synthesis.";
      return;
    }

    if (!textToSpeak.trim()) {
      ttsError = "Please enter text to speak.";
      ttsStatus = "Error: No text provided.";
      return;
    }

    ttsStatus = "Synthesizing speech...";
    try {
      // Invoke the Rust command for TTS
      const audioDataArray = await invoke<number[]>("synthesize_speech", { text: textToSpeak });
      const audioData = new Uint8Array(audioDataArray);
      
      // Convert audio data to Blob for playback
      const audioBlob = new Blob([audioData], { type: "audio/wav" }); // Assuming WAV format from VietTTS
      const audioUrl = URL.createObjectURL(audioBlob);
      
      // Play the audio
      const audio = new Audio(audioUrl);
      audio.onplay = () => {
        isPlaying = true;
        ttsStatus = "Playing audio...";
      };
      audio.onended = () => {
        isPlaying = false;
        ttsStatus = "Speech playback completed.";
        URL.revokeObjectURL(audioUrl); // Clean up URL
      };
      audio.onerror = (error) => {
        isPlaying = false;
        ttsError = `Audio playback error: ${error instanceof Error ? error.message : 'Unknown error'}`;
        ttsStatus = "Error during playback.";
        URL.revokeObjectURL(audioUrl);
      };
      
      await audio.play();
    } catch (error) {
      console.error("TTS Process Error:", error);
      ttsError = `TTS Error: ${error}`;
      ttsStatus = "Error during speech synthesis.";
    }
  }

</script>

<main class="container">
  <h1>Mivis Desktop Assistant</h1>
<!-- 
  Existing Greet Section 
  <div class="row">
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://kit.svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte-kit" alt="SvelteKit Logo" />
    </a>
  </div>
  <p>Click on the Tauri, Vite, and SvelteKit logos to learn more.</p>


  <hr style="margin: 20px 0;">

  <!-- STT Section 
  <h2>Speech-to-Text</h2>
  <div class="row">
    {#if !isRecording}
      <button onclick={startRecording}>Start Recording</button>
    {:else}
      <button onclick={stopRecording}>Stop Recording</button>
    {/if}
  </div>
  <p>Status: {sttStatus}</p>
  {#if transcription}
    <p>Transcription: {transcription}</p>
  {/if}
  {#if sttError}
    <p style="color: red;">Error: {sttError}</p>
  {/if}

  <hr style="margin: 20px 0;">

  <!-- TTS Section 
  <h2>Text-to-Speech</h2>
  <div class="row">
    <input type="text" bind:value={textToSpeak} placeholder="Enter text to speak" style="width: 300px; padding: 0.6em; margin-right: 10px;" />
    <button onclick={startSpeaking} disabled={isPlaying || !textToSpeak.trim()}>Speak</button>
  </div>
  <p>Status: {ttsStatus}</p>
  {#if ttsError}
    <p style="color: red;">Error: {ttsError}</p>
  {/if}

  <hr style="margin: 20px 0;"> -->

  <!-- AI Chat Section -->
  <h2>AI Chat</h2>
  <ChatBox />

</main>

<style>
/* .logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.svelte-kit:hover {
  filter: drop-shadow(0 0 2em #ff3e00);
} */

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

/* .logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
} */

/* .logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
} */

/* .row {
  display: flex;
  justify-content: center;
} */

/* a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
} */

/* a:hover {
  color: #535bf2;
} */

h1 {
  text-align: center;
}


/* button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}


button {
  outline: none;
} */


@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  /* a:hover {
    color: #24c8db;
  }


  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  } */
}

</style>
