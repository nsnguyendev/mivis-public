/**
 * @file vietTTSClient.ts
 * @description Client for interacting with the VietTTS API service.
 * This client facilitates communication with the VietTTS Docker service
 * running at http://localhost:8298 to perform text-to-speech synthesis.
 */

const BASE_URL = 'http://localhost:8298/v1';
const API_TOKEN = 'viet-tts'; // As per VietTTS documentation

interface Voice {
  id: string;
  name: string;
  // Add other voice properties if available from the API
}

/**
 * Fetches the list of available voices from the VietTTS service.
 * @async
 * @function listVoices
 * @returns {Promise<Voice[]>} A promise that resolves to an array of available voices.
 * @throws {Error} If the API request fails or returns an unexpected status.
 *
 * @example
 * async function example() {
 *   try {
 *     const voices = await listVoices();
 *     console.log('Available voices:', voices);
 *   } catch (error) {
 *     console.error('Failed to list voices:', error);
 *   }
 * }
 */
export async function listVoices(): Promise<Voice[]> {
  try {
    const response = await fetch(`${BASE_URL}/voices`, {
      method: 'GET',
      headers: {
        'Authorization': `Bearer ${API_TOKEN}`,
      },
    });

    if (!response.ok) {
      const errorBody = await response.text();
      throw new Error(`Failed to list voices: ${response.status} ${response.statusText} - ${errorBody}`);
    }

    const voices = await response.json();
    return voices as Voice[];
  } catch (error) {
    console.error(`[ERROR] listVoices: ${error instanceof Error ? error.message : String(error)}`);
    throw new Error(`Failed to retrieve voice list: ${error instanceof Error ? error.message : String(error)}`);
  }
}

/**
 * Synthesizes speech from the given text using a specified voice.
 * @async
 * @function synthesizeSpeech
 * @param {string} text The text to synthesize.
 * @param {string} voice The ID or name of the voice to use for synthesis.
 * @returns {Promise<ArrayBuffer>} A promise that resolves to an ArrayBuffer containing the audio data.
 * @throws {Error} If the API request fails, returns an unexpected status, or if audio data cannot be retrieved.
 *
 * @example
 * async function exampleSynthesis() {
 *   try {
 *     const audioData = await synthesizeSpeech('Xin chào Việt Nam.', 'son-tung-mtp');
 *     // Assuming you have a way to play or save the ArrayBuffer (e.g., in Node.js or browser)
 *     // fs.writeFileSync('speech.wav', Buffer.from(audioData));
 *     console.log('Speech synthesized successfully, data length:', audioData.byteLength);
 *   } catch (error) {
 *     console.error('Failed to synthesize speech:', error);
 *   }
 * }
 */
export async function synthesizeSpeech(text: string, voice: string): Promise<ArrayBuffer> {
  try {
    const response = await fetch(`${BASE_URL}/audio/speech`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${API_TOKEN}`,
      },
      body: JSON.stringify({
        model: 'tts-1', // As per VietTTS documentation for OpenAI compatibility
        input: text,
        voice: voice,
      }),
    });

    if (!response.ok) {
      const errorBody = await response.text();
      throw new Error(`Failed to synthesize speech: ${response.status} ${response.statusText} - ${errorBody}`);
    }

    // The response should be audio data, e.g., 'audio/wav' or 'audio/mpeg'
    // The OpenAI client example uses .arrayBuffer()
    const audioData = await response.arrayBuffer();
    return audioData;
  } catch (error) {
    console.error(`[ERROR] synthesizeSpeech: ${error instanceof Error ? error.message : String(error)}`);
    throw new Error(`Failed to synthesize speech: ${error instanceof Error ? error.message : String(error)}`);
  }
}
