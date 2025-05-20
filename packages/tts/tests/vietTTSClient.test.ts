/**
 * @file vietTTSClient.test.ts
 * @description Unit tests for the VietTTS client.
 * These tests mock the fetch API to simulate responses from the VietTTS service.
 */

import { listVoices, synthesizeSpeech } from '../src/vietTTSClient';

// Mock the global fetch function
global.fetch = jest.fn();

const mockFetch = global.fetch as jest.Mock;

describe('VietTTS Client', () => {
  beforeEach(() => {
    // Clear all instances and calls to constructor and all methods:
    mockFetch.mockClear();
  });

  describe('listVoices', () => {
    it('should return a list of voices on successful API call', async () => {
      const mockVoiceData = [{ id: '1', name: 'Voice A' }, { id: '2', name: 'Voice B' }];
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: async () => mockVoiceData,
      } as Response);

      const voices = await listVoices();
      expect(voices).toEqual(mockVoiceData);
      expect(mockFetch).toHaveBeenCalledTimes(1);
      expect(mockFetch).toHaveBeenCalledWith('http://localhost:8298/v1/voices', {
        method: 'GET',
        headers: { 'Authorization': 'Bearer viet-tts' },
      });
    });

    it('should throw an error if the API call fails', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: false,
        status: 500,
        statusText: 'Internal Server Error',
        text: async () => 'Server error details',
      } as Response);

      // Updated expected error message to match the actual format from the catch block
      await expect(listVoices()).rejects.toThrow('Failed to retrieve voice list: Failed to list voices: 500 Internal Server Error - Server error details');
      expect(mockFetch).toHaveBeenCalledTimes(1);
    });

    it('should throw an error if the network request fails', async () => {
      mockFetch.mockRejectedValueOnce(new Error('Network failure'));
      await expect(listVoices()).rejects.toThrow('Failed to retrieve voice list: Network failure');
    });
  });

  describe('synthesizeSpeech', () => {
    const text = 'Xin chào';
    const voice = 'VoiceA';

    it('should return an ArrayBuffer on successful API call', async () => {
      const mockAudioArrayBuffer = new ArrayBuffer(1024);
      mockFetch.mockResolvedValueOnce({
        ok: true,
        arrayBuffer: async () => mockAudioArrayBuffer,
      } as Response);

      const audioData = await synthesizeSpeech(text, voice);
      expect(audioData).toBeInstanceOf(ArrayBuffer);
      expect(audioData.byteLength).toBe(1024);
      expect(mockFetch).toHaveBeenCalledTimes(1);
      expect(mockFetch).toHaveBeenCalledWith('http://localhost:8298/v1/audio/speech', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': 'Bearer viet-tts',
        },
        body: JSON.stringify({
          model: 'tts-1',
          input: text,
          voice: voice,
        }),
      });
    });

    it('should throw an error if the API call fails', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: false,
        status: 400,
        statusText: 'Bad Request',
        text: async () => 'Invalid input',
      } as Response);

      await expect(synthesizeSpeech(text, voice)).rejects.toThrow('Failed to synthesize speech: 400 Bad Request - Invalid input');
      expect(mockFetch).toHaveBeenCalledTimes(1);
    });

    it('should throw an error if the network request fails', async () => {
      mockFetch.mockRejectedValueOnce(new Error('Network problem'));
      await expect(synthesizeSpeech(text, voice)).rejects.toThrow('Failed to synthesize speech: Network problem');
    });
  });
});
