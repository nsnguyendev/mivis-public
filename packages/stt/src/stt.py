import speech_recognition as sr
import sys
import os # Import the os module
import codecs # Import codecs for encoding handling
import wave # Import the wave module

# Explicitly set stdout and stderr encoding to UTF-8
sys.stdout = codecs.getwriter("utf-8")(sys.stdout.detach())
sys.stderr = codecs.getwriter("utf-8")(sys.stderr.detach())


def transcribe_audio(audio_path):
    # Check sample rate
    try:
        with wave.open(audio_path, 'rb') as wf:
            frame_rate = wf.getframerate()
            if frame_rate != 16000:
                error_message = f"Lỗi: Tốc độ mẫu âm thanh phải là 16000 Hz, nhưng nhận được {frame_rate} Hz."
                print(error_message, file=sys.stderr) # Print to stderr
                return None # Indicate failure
    except wave.Error as e:
        error_message = f"Lỗi khi đọc file WAV: {e}"
        print(error_message, file=sys.stderr) # Print to stderr
        return None # Indicate failure
    except FileNotFoundError:
         # This case is handled by SpeechRecognition, but good to have a check here too
         pass # Let SpeechRecognition handle FileNotFoundError for consistency


    recognizer = sr.Recognizer()
    # Use a try-except block around the AudioFile context manager
    try:
        with sr.AudioFile(audio_path) as source:
            audio = recognizer.record(source)
            try:
                # Construct the absolute path to the model directory
                script_dir = os.path.dirname(__file__)
                model_relative_path = "../models/PhoWhisper-ct2-FasterWhisper/PhoWhisper-base-ct2-fasterWhisper"
                model_abs_path = os.path.abspath(os.path.join(script_dir, model_relative_path))

                # Use the absolute path for the model
                result = recognizer.recognize_faster_whisper(audio, language="vi", model=model_abs_path)
                return result
            except sr.UnknownValueError:
                return "Không nhận diện được"
            except sr.RequestError as e:
                return f"Lỗi: {e}"
    except FileNotFoundError:
        # Handle FileNotFoundError specifically if AudioFile raises it
        error_message = f"Lỗi: Không tìm thấy file âm thanh tại đường dẫn {audio_path}"
        print(error_message, file=sys.stderr) # Print to stderr
        sys.stderr.flush() # Explicitly flush stderr
        return None # Indicate failure
    except Exception as e:
        # Catch any other unexpected errors during file processing
        error_message = f"Lỗi xử lý file âm thanh: {e}"
        print(error_message, file=sys.stderr) # Print to stderr
        sys.stderr.flush() # Explicitly flush stderr
        return None # Indicate failure


if __name__ == "__main__":
    if len(sys.argv) > 1:
        audio_path = sys.argv[1]
        try:
            transcription = transcribe_audio(audio_path)
            if transcription is not None: # Only print if transcription was successful
                print(transcription)
            else:
                # If transcribe_audio returns None without raising an error (shouldn't happen now)
                sys.exit(1) # Exit with error code
        except Exception as e:
            # Catch errors raised by transcribe_audio and exit with error code
            # Error message should have already been printed to stderr
            sys.exit(1)
    else:
        print("Vui lòng cung cấp đường dẫn file audio, ví dụ: python stt.py 'D:/path/to/sample.wav'")
        sys.exit(1) # Exit with error code if no argument provided
