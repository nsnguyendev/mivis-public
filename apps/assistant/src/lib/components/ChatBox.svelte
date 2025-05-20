<script lang="ts">
  import { writable, get } from 'svelte/store';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { onMount, onDestroy } from 'svelte';

  // Define message interface
  interface Message {
    role: 'user' | 'assistant';
    content: string;
    source?: 'text' | 'voice';
    timestamp?: number; // For unique keys if needed
  }

  type ProcessingStage = 'IDLE' | 'TRANSCRIBING' | 'PROCESSING_API' | 'SYNTHESIZING_VOICE';

  interface ProcessingStageUpdatePayload {
    stage: ProcessingStage;
    message?: string;
  }

  // Stores for managing chat state
  const messages = writable<Message[]>([]); // Chat history
  const isLoading = writable<boolean>(false); // General loading state for API calls
  const error = writable<string | null>(null); // For actual errors
  const isRecording = writable<boolean>(false);
  const isPlayingTTS = writable<boolean>(false);
  const ttsEnabled = writable<boolean>(true);

  const currentProcessingStage = writable<ProcessingStage>('IDLE');
  const currentUserBubbleContent = writable<string | null>(null);
  const currentAssistantBubbleContent = writable<string | null>(null);
  const statusAreaMessage = writable<string | null>(null); // For status updates like "Transcribing..."

  let unlisten: (() => void) | null = null;

  onMount(async () => {
    unlisten = await listen<ProcessingStageUpdatePayload>('processing_stage_update', (event) => {
      const { stage, message } = event.payload;
      currentProcessingStage.set(stage);
      error.set(null); // Clear previous errors on new stage

      switch (stage) {
        case 'IDLE':
          statusAreaMessage.set(null);
          // currentUserBubbleContent.set(null); // Cleared after turn completion
          // currentAssistantBubbleContent.set(null); // Cleared after turn completion
          isLoading.set(false);
          break;
        case 'TRANSCRIBING':
          statusAreaMessage.set(message || 'Transcribing voice...');
          currentUserBubbleContent.set(message || 'Transcribing voice...');
          currentAssistantBubbleContent.set(null);
          isLoading.set(true);
          break;
        case 'PROCESSING_API':
          statusAreaMessage.set(message || 'Processing request...');
          // User bubble should have transcribed text, set by handleTranscribedText
          currentAssistantBubbleContent.set(message || 'Processing request...');
          isLoading.set(true);
          break;
        case 'SYNTHESIZING_VOICE':
          statusAreaMessage.set(message || 'Synthesizing voice...');
          // Assistant bubble should have API response, set by handleApiResponse
          // Potentially start streaming text here if API response is already available
          isLoading.set(true);
          break;
      }
    });
  });

  onDestroy(() => {
    if (unlisten) {
      unlisten();
    }
  });

  // Function to send message to backend (for text input)
  async function sendTextMessage(input: string) {
    if (!input.trim()) return;
    
    const userMessage: Message = { role: 'user', content: input, source: 'text', timestamp: Date.now() };
    messages.update(msgs => [...msgs, userMessage]);
    currentUserBubbleContent.set(input); // Show user message in their bubble immediately
    currentAssistantBubbleContent.set(null); // Clear assistant bubble

    // This will trigger PROCESSING_API event from backend
    try {
      isLoading.set(true); // Manually set loading for text messages before backend event
      statusAreaMessage.set("Processing request..."); // Show status for text messages
      currentAssistantBubbleContent.set("Processing request...");


      // Create a snapshot of messages to send to LLM
      // The system prompt is added on the backend.
      // We only send the current conversation history.
      const messagesForLLM = [...get(messages)]; 
      // Remove the last message if it's the one we just added, to avoid sending it to itself if invoke_llm_chat expects only prior history
      // However, invoke_llm_chat in Rust takes Vec<Message>, implying it wants the full context including current user query.
      // The backend prepends system prompt.

      const apiResponse = await invoke<string>('invoke_llm_chat', {
        messages: messagesForLLM // Send current messages array
      });
      handleApiResponse(apiResponse, 'text');
    } catch (e: unknown) {
      error.set(`Failed to get response: ${String(e)}`);
      currentProcessingStage.set('IDLE');
      statusAreaMessage.set(null);
      currentAssistantBubbleContent.set(null); // Clear assistant bubble on error
      isLoading.set(false);
    }
  }

  // Auto-scroll to latest message
  let chatContainer: HTMLElement | undefined;
  $: if (chatContainer) {
    chatContainer.scrollTop = chatContainer.scrollHeight;
  }
  
  let inputValue = '';
  let mediaRecorder: MediaRecorder | null = null;
  let audioChunks: Blob[] = [];

  // Function to handle API response
  async function handleApiResponse(apiResponse: string, source: 'text' | 'voice') {
    // Stop showing "Processing request..." in assistant bubble
    if (get(currentProcessingStage) === 'PROCESSING_API') {
        // This check might be redundant if SYNTHESIZING_VOICE stage is set quickly
    }
    
    // Simulate streaming text display for assistant bubble
    // For now, just set it directly. Streaming can be added later.
    currentAssistantBubbleContent.set(apiResponse);
    statusAreaMessage.set(null); // Clear "Processing..." or "Synthesizing..."

    if (!$ttsEnabled) {
      const assistantMessage: Message = { role: 'assistant', content: apiResponse, timestamp: Date.now() };
      messages.update(msgs => [...msgs, assistantMessage]);

      currentUserBubbleContent.set(null);
      currentAssistantBubbleContent.set(null);
      currentProcessingStage.set('IDLE');  
      isLoading.set(false);
    } 
    else if ($ttsEnabled) {
      await playTTS(apiResponse); // playTTS will set its own stages 

    } else {
      // If TTS is not playing, the turn ends here for text or voice-no-tts
      currentUserBubbleContent.set(null);
      currentAssistantBubbleContent.set(null);
      currentProcessingStage.set('IDLE');
      isLoading.set(false);
    }
  }


  // Function to handle transcribed text from STT
  async function handleTranscribedText(transcribedText: string) {
    currentUserBubbleContent.set(transcribedText);
    const userMessage: Message = { role: 'user', content: transcribedText, source: 'voice', timestamp: Date.now() };
    messages.update(msgs => [...msgs, userMessage]);

    // This will trigger PROCESSING_API event from backend
    try {
      // Create a snapshot of messages to send to LLM
      const messagesForLLM = [...get(messages)];
      const apiResponse = await invoke<string>('invoke_llm_chat', {
        messages: messagesForLLM
      });
      handleApiResponse(apiResponse, 'voice');
    } catch (e: unknown) {
      error.set(`Failed to get response: ${String(e)}`);
      currentProcessingStage.set('IDLE');
      statusAreaMessage.set(null);
      currentUserBubbleContent.set(null); // Clear user bubble on error
      currentAssistantBubbleContent.set(null); // Clear assistant bubble on error
      isLoading.set(false);
    }
  }


  // Function to start recording audio
  async function startRecording() {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      // Attempt to record in WAV format if supported by the browser
      const options = { mimeType: 'audio/wav' };
      if (!MediaRecorder.isTypeSupported(options.mimeType)) {
        console.warn(`${options.mimeType} is not supported, falling back to default.`);
        // Fallback to default or try 'audio/webm;codecs=pcm' or other options if needed
        // For now, let it use browser default if WAV is not supported. Backend expects WAV.
        // This might require a client-side conversion library (e.g., using WASM) for full compatibility.
        mediaRecorder = new MediaRecorder(stream);
      } else {
        mediaRecorder = new MediaRecorder(stream, options);
      }
      
      audioChunks = [];
      
      mediaRecorder.ondataavailable = (event) => {
        audioChunks.push(event.data);
      };
      
      mediaRecorder.onstop = async () => {
        isRecording.set(false); // Moved here, stop shows before transcription starts
        const audioBlob = new Blob(audioChunks, { type: mediaRecorder?.mimeType || 'audio/wav' });
        const arrayBuffer = await audioBlob.arrayBuffer();
        const uint8Array = new Uint8Array(arrayBuffer);
        
        // Backend will emit TRANSCRIBING via invoke_stt_transcription
        try {
          const transcribedText = await invoke<string>('invoke_stt_transcription', { audioData: Array.from(uint8Array) }); 
          // Event listener for 'TRANSCRIBING' should have updated currentUserBubbleContent
          // Now handle the transcribed text
          await handleTranscribedText(transcribedText);
        } catch (e: unknown) {
          error.set(`Failed to transcribe audio: ${String(e)}`);
          currentProcessingStage.set('IDLE'); // Reset stage on STT error
          statusAreaMessage.set(null);
          currentUserBubbleContent.set(null);
          isLoading.set(false);
        }
      };
      
      mediaRecorder.start();
      isRecording.set(true);
      // Backend will emit TRANSCRIBING stage, which updates statusAreaMessage and currentUserBubbleContent
      // So, no need to set statusAreaMessage.set('Recording...') here.
      
      // Timeout after 10 seconds
      setTimeout(() => {
        if (mediaRecorder && mediaRecorder.state === 'recording') {
          mediaRecorder.stop();
          // statusAreaMessage.set('Recording timed out after 10 seconds'); // This might conflict with STT status
        }
      }, 100000);
    } catch (e: unknown) {
      error.set(`Failed to access microphone: ${String(e)}`);
      isRecording.set(false);
      currentProcessingStage.set('IDLE'); // Reset stage on mic error
      statusAreaMessage.set(null);
      isLoading.set(false);
    }
  }

  // Function to stop recording
  function stopRecording() {
    if (mediaRecorder && mediaRecorder.state === 'recording') {
      mediaRecorder.stop();
      // Stream tracks are stopped by MediaRecorder itself on stop usually,
      // but explicit stop can be good for quick resource release.
      mediaRecorder.stream.getTracks().forEach(track => track.stop());
    }
    // isRecording.set(false); // Set in onstop to ensure it's false after processing
  }

  // Function to play TTS audio
  async function playTTS(text: string) {
    if (!get(ttsEnabled)) { // Use get() for store value
        // If TTS is disabled but was part of a voice flow, end the turn.
        currentUserBubbleContent.set(null);
        currentAssistantBubbleContent.set(null);
        currentProcessingStage.set('IDLE');
        isLoading.set(false);
        return;
    }

    isPlayingTTS.set(true);
    // Backend will emit SYNTHESIZING_VOICE stage
    try {
      const audioData = await invoke<number[]>('synthesize_speech', { text });
      const uint8Array = new Uint8Array(audioData);
      const audioBlob = new Blob([uint8Array], { type: 'audio/wav' });
      const audioUrl = URL.createObjectURL(audioBlob);
      const audio = new Audio(audioUrl);
      
      audio.onended = () => {
        isPlayingTTS.set(false);
        URL.revokeObjectURL(audioUrl);
        // TTS finished, end of turn
        currentUserBubbleContent.set(null);
        currentAssistantBubbleContent.set(null);
        currentProcessingStage.set('IDLE');
        statusAreaMessage.set(null);
        isLoading.set(false);
      };
      await audio.play();
      const assistantMessage: Message = { role: 'assistant', content: text, timestamp: Date.now() };
      messages.update(msgs => [...msgs, assistantMessage]);
    } catch (e: unknown) {
      error.set(`Failed to play TTS: ${String(e)}`);
      isPlayingTTS.set(false);
      // TTS failed, end of turn
      currentUserBubbleContent.set(null);
      currentAssistantBubbleContent.set(null);
      currentProcessingStage.set('IDLE');
      statusAreaMessage.set(null);
      isLoading.set(false);
    }
  }

  // Function to toggle TTS playback
  function toggleTTSEnabled() {
    ttsEnabled.update(enabled => !enabled);
  }
</script>

<div class="chat-box">
  <div class="chat-container" bind:this={chatContainer} role="log" aria-label="Chat history">
    {#each $messages as msg (msg.timestamp || msg.content)}
      <div class="message {msg.role === 'user' ? 'user-message' : 'assistant-message'}">
        <div class="message-content">{msg.content}</div>
        {#if msg.source === 'voice'}
          <span class="source-indicator">(Voice)</span>
        {/if}
      </div>
    {/each}
  </div>
  
  {#if $error}
    <div class="error-notification" role="alert">{$error}</div>
  {/if}

  {#if $statusAreaMessage && $currentProcessingStage !== 'IDLE'}
    <div class="status-notification" role="status">
      {$statusAreaMessage}
      {#if $isLoading && ($currentProcessingStage === 'TRANSCRIBING' || $currentProcessingStage === 'PROCESSING_API' || $currentProcessingStage === 'SYNTHESIZING_VOICE')}
        <span class="loading-spinner"></span> <!-- Simple spinner, replace with actual SVG/CSS spinner -->
      {/if}
    </div>
  {/if}
  
  <div class="input-container">
    <button
      class="voice-button"
      on:click={() => get(isRecording) ? stopRecording() : startRecording()}
      disabled={get(isLoading) || get(isPlayingTTS)}
      aria-label="Toggle voice input"
    >
      {#if $isRecording}
        ⏹️ Stop
      {:else}
        🎤 Record
      {/if}
    </button>
    <input
      type="text"
      bind:value={inputValue}
      on:keypress={(e) => e.key === 'Enter' && (sendTextMessage(inputValue), inputValue = '')}
      placeholder="Type your message..."
      disabled={$isLoading || $isRecording || $isPlayingTTS}
      aria-label="Chat input field"
    />
    <button
      class="send-button"
      on:click={() => { sendTextMessage(inputValue); inputValue = ''; }}
      disabled={$isLoading || $isRecording || $isPlayingTTS || !inputValue.trim()}
      aria-label="Send message"
    >
      {#if $isLoading && $currentProcessingStage !== 'IDLE' && $currentProcessingStage !== 'TRANSCRIBING'} <!-- Show "Sending..." only for API processing -->
        Sending...
      {:else}
        Send
      {/if}
    </button>
    <button
      class="voice-output-indicator"
      on:click={toggleTTSEnabled}
      aria-label="Toggle voice output"
    >
      {#if $isPlayingTTS}
        🔇 <!-- Playing TTS -->
      {:else if $ttsEnabled}
        🔊 <!-- TTS Enabled, not playing -->
      {:else}
        🔈 <!-- TTS Disabled -->
      {/if}
    </button>
  </div>
</div>

<style>
  .chat-box {
    display: flex;
    flex-direction: column;
    height: 100%;
    max-width: 800px;
    margin: 0 auto;
    padding: 1rem;
    background-color: #f5f5f5;
    border-radius: 8px;
  }

  .chat-container {
    flex: 1;
    overflow-y: auto;
    padding: 1rem;
    margin-bottom: 1rem;
    background-color: white;
    border: 1px solid #e0e0e0;
    border-radius: 4px;
    display: flex;
    flex-direction: column;
  }

  .message {
    margin-bottom: 1rem;
    padding: 0.75rem 1rem; /* Increased padding */
    border-radius: 18px; /* More rounded bubbles */
    max-width: 75%; /* Slightly less wide */
    word-wrap: break-word;
    line-height: 1.4;
  }
  
  .user-message {
    background-color: #007bff;
    color: white;
    margin-left: auto;
    align-self: flex-end; /* Align to right */
  }

  .assistant-message {
    background-color: #e9ecef;
    color: black;
    margin-right: auto;
    align-self: flex-start; /* Align to left */
  }

  .source-indicator {
    font-size: 0.7rem;
    opacity: 0.7;
    margin-left: 0.5rem;
    display: inline-block; /* Ensure it's on the same line if space allows */
  }

  .input-container {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding-top: 0.5rem;
    border-top: 1px solid #e0e0e0;
  }

  .voice-button,
  .send-button,
  .voice-output-indicator {
    padding: 0.6rem 1rem; /* Adjusted padding */
    border: none;
    border-radius: 20px; /* More rounded buttons */
    cursor: pointer;
    background-color: #007bff;
    color: white;
    transition: background-color 0.2s;
    font-size: 0.9rem;
  }
  
  .voice-button:hover:not(:disabled),
  .send-button:hover:not(:disabled),
  .voice-output-indicator:hover:not(:disabled) {
    background-color: #0056b3;
  }


  .voice-button:disabled,
  .send-button:disabled,
  .voice-output-indicator:disabled {
    background-color: #cccccc;
    cursor: not-allowed;
  }

  input[type="text"] {
    flex: 1;
    padding: 0.75rem; /* Increased padding */
    border: 1px solid #ced4da; /* Slightly different border */
    border-radius: 20px; /* More rounded input */
    outline: none;
    font-size: 1rem;
  }

  input[type="text"]:focus {
    border-color: #80bdff;
    box-shadow: 0 0 0 0.2rem rgba(0, 123, 255, 0.25);
  }


  input:disabled {
    background-color: #e9ecef; /* Consistent disabled background */
    cursor: not-allowed;
  }

  .error-notification, .status-notification {
    padding: 0.75rem 1rem;
    margin-bottom: 1rem;
    border-radius: 4px;
    text-align: center;
  }

  .error-notification {
    background-color: #f8d7da;
    color: #721c24; /* Darker red for better contrast */
    border: 1px solid #f5c6cb;
  }
  
  .status-notification {
    background-color: #d1ecf1;
    color: #0c5460;
    border: 1px solid #bee5eb;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .voice-output-indicator {
    font-size: 1.2rem;
  }

  .loading-spinner {
    display: inline-block;
    width: 1em;
    height: 1em;
    border: 2px solid currentColor;
    border-right-color: transparent;
    border-radius: 50%;
    animation: spinner-border .75s linear infinite;
    margin-left: 0.5rem;
  }

  @keyframes spinner-border {
    to { transform: rotate(360deg); }
  }
  
</style>
