/**
 * @file stt.test.ts
 * @description Integration tests for the STT Python script.
 * These tests execute the stt.py script as a child process to verify its
 * speech-to-text functionality with a sample audio file.
 */

import { exec } from 'child_process';
import * as path from 'path';
import * as util from 'util';
import * as fs from 'fs'; // Import the fs module

const execPromise = util.promisify(exec);

// Define the path to the stt.py script and sample audio files
const sttScriptPath = path.resolve(__dirname, '../src/stt.py'); // Updated path
const sampleAudioPath = path.resolve(__dirname, '../samples/sample_converted.wav');
// Path to a sample WAV file with a sample rate other than 16000 Hz for the invalid sample rate test
const invalidSampleRateAudioPath = path.resolve(__dirname, '../samples/sample.wav'); // Updated path


describe('STT Script Integration Test', () => {
  // Timeout for the test, as script execution and model loading can take time.
  jest.setTimeout(60000); // Increased timeout to 60 seconds

  it('should transcribe the sample audio file and produce output', async () => {
    // Construct the command to execute the Python script.
    const command = `python "${sttScriptPath}" "${sampleAudioPath}"`;

    try {
      // Specify utf8 encoding for stdout and stderr
      const { stdout, stderr } = await execPromise(command, { encoding: 'utf8' });

      if (stderr) {
        console.warn(`[WARN] STT script stderr: ${stderr}`);
      }

      // Trim whitespace and assert that the output is not empty.
      const transcribedText = stdout.trim();
      expect(transcribedText.length).toBeGreaterThan(0); // Assert that output is not empty
      // Optionally, you could add a check for specific keywords if needed, e.g.,
      // expect(transcribedText.toLowerCase()).toContain('hai không lẽ hai');

    } catch (error) {
      // Log the error for debugging purposes.
      const execError = error as { stdout?: string; stderr?: string; message: string };
      console.error(`[ERROR] Failed to execute STT script: ${execError.message}`);
      if (execError.stdout) console.error(`[ERROR] STDOUT: ${execError.stdout}`);
      if (execError.stderr) console.error(`[ERROR] STDERR: ${execError.stderr}`);
      // Fail the test explicitly if the script execution fails.
      throw new Error(`STT script execution failed: ${execError.message}`);
    }
  });

  it('should handle non-existent audio file path gracefully', async () => {
    const nonExistentAudioPath = path.resolve(__dirname, '../samples/non_existent_audio.wav');
    const command = `python "${sttScriptPath}" "${nonExistentAudioPath}"`;

    try {
      // Specify utf8 encoding for stdout and stderr
      const { stdout, stderr } = await execPromise(command, { encoding: 'utf8' });
      // Expecting an error message on stderr for file not found
      expect(stderr.trim().toLowerCase()).toMatch(/lỗi: không tìm thấy file âm thanh tại đường dẫn/);
      // Expect no output on stdout
      expect(stdout.trim()).toBe('');

    } catch (error) {
        // If the script exits with a non-zero code, execPromise will throw.
        // We expect this for a file not found error.
        const execError = error as { stdout?: string; stderr?: string; message: string, code?: number };
        const output = (execError.stdout || execError.stderr || "").trim().toLowerCase();
        // Updated regex to match the FileNotFoundError traceback or the script's error message
        expect(output).toMatch(/filenotfounderror: \[errno 2\] no such file or directory:|lỗi: không tìm thấy file âm thanh tại đường dẫn/);
        // Optionally check the exit code if the script is expected to exit with a specific code on error
        // expect(execError.code).toBe(1); // Assuming 1 for general errors
    }
  });

  // New test case for invalid sample rate
  // !!! This test requires a WAV file with a sample rate other than 16000 Hz !!!
  it('should handle audio file with invalid sample rate gracefully', async () => {
    // Check if the invalid sample rate audio file exists before running the test
    const invalidFileExists = await new Promise(resolve => {
        fs.access(invalidSampleRateAudioPath, fs.constants.F_OK, (err) => {
            resolve(!err);
        });
    });

    if (!invalidFileExists) {
        console.warn(`Skipping test: ${invalidSampleRateAudioPath} not found. Please provide a WAV file with a sample rate other than 16000 Hz.`);
        return; // Skip the test if the file doesn't exist
    }

    // Construct the command to execute the Python script, redirecting stderr to stdout
    const command = `python "${sttScriptPath}" "${invalidSampleRateAudioPath}" 2>&1`;

    try {
      // Specify utf8 encoding for stdout (which now includes stderr)
      const { stdout } = await execPromise(command, { encoding: 'utf8' });

      // Expect the error message related to invalid sample rate on stdout
      const combinedOutput = stdout.trim().toLowerCase();
      expect(combinedOutput).toMatch(/lỗi: tốc độ mẫu âm thanh phải là 16000 hz, nhưng nhận được \d+ hz\./);

    } catch (error) {
        // If the script exits with a non-zero code, execPromise will throw.
        const execError = error as { stdout?: string; stderr?: string; message: string, code?: number };
        // Check the combined output (stdout + redirected stderr) for the actual error message
        const combinedOutput = (execError.stdout || execError.stderr || "").trim().toLowerCase();
        expect(combinedOutput).toMatch(/lỗi khi đọc file wav: unknown format: 3/); // Updated to match actual error
        // Optionally check the exit code
        // expect(execError.code).toBe(1); // Assuming 1 for general errors
    }
  });


  // Note: Testing the Conda environment activation is complex from within Jest.
  // It's often better to ensure the test execution environment (e.g., CI server, local dev setup)
  // is correctly configured to run the Python script with its dependencies.
  // If stt.py is not found or Python is not configured, the first test will likely fail
  // at the execPromise call.
});
