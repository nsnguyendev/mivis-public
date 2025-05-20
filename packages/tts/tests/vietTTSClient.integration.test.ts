/**
 * @file vietTTSClient.integration.test.ts
 * @description Integration tests for the VietTTS client.
 * These tests verify interaction with the live VietTTS Docker service.
 * Requires the Docker service to be running at http://localhost:8298.
 */

import { listVoices, synthesizeSpeech } from '../src/vietTTSClient';

// Increase timeout for integration tests as they involve network calls
jest.setTimeout(30000); // 30 seconds

describe('VietTTS Client Integration Tests', () => {
  // Note: These tests require the VietTTS Docker service to be running.
  // They are marked as integration tests and should be run separately from unit tests.

  describe('listVoices (Integration)', () => {
    it('should successfully fetch a list of voices from the live service', async () => {
      try {
        const voices = await listVoices();

        // Assert that the call was successful and returned an array
        expect(Array.isArray(voices)).toBe(true);
        // Assert that the array is not empty (assuming the service always returns voices)
        expect(voices.length).toBeGreaterThan(0);
        // Optionally, check for a known voice ID if applicable
        // expect(voices.some(voice => voice.id === 'cdteam')).toBe(true);

      } catch (error) {
        // If the service is not running or the call fails, this test will fail.
        // Log the error for debugging.
        console.error(`[ERROR] Integration test failed for listVoices: ${error instanceof Error ? error.message : String(error)}`);
        throw error; // Re-throw to fail the test
      }
    });
  });

  describe('synthesizeSpeech (Integration)', () => {
    // Use a known valid voice ID from the VietTTS documentation
    const testVoiceId = 'cdteam';
    const testText = 'Xin chào Việt Nam.';

    it('should successfully synthesize speech and return audio data from the live service', async () => {
      try {
        const audioData = await synthesizeSpeech(testText, testVoiceId);

        // Assert that the call was successful and returned an ArrayBuffer
        expect(audioData).toBeInstanceOf(ArrayBuffer);
        // Assert that the audio data is not empty
        expect(audioData.byteLength).toBeGreaterThan(0);

      } catch (error) {
        // If the service is not running, the voice ID is invalid, or the call fails, this test will fail.
        // Log the error for debugging.
        console.error(`[ERROR] Integration test failed for synthesizeSpeech: ${error instanceof Error ? error.message : String(error)}`);
        throw error; // Re-throw to fail the test
      }
    });
  });

  // Add more integration test cases here as needed, e.g., testing different voices, error conditions, etc.
});
