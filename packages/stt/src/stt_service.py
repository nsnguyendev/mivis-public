import sys
import os
import soundfile as sf
import numpy as np
import subprocess
import shutil
from datetime import datetime
from flask import Flask, request, jsonify
# Assuming stt.py contains the transcribe_audio function
# Use absolute import or adjust based on how the service is run
try:
    from stt import transcribe_audio
except ImportError:
    # Fallback for running as a script directly
    sys.path.append(os.path.join(os.path.dirname(__file__)))
    from stt import transcribe_audio


app = Flask(__name__)

# @app.before_request
# def log_request():
#     from datetime import datetime
#     log_dir = os.path.join(os.path.dirname(__file__), 'debug_logs')
#     os.makedirs(log_dir, exist_ok=True)
#     timestamp = datetime.now().strftime("%Y%m%d_%H%M%S_%f")
#     log_file_path = os.path.join(log_dir, f"request_{timestamp}.txt")
    
#     with open(log_file_path, 'a') as f:
#         f.write(f"[{datetime.now().strftime('%Y-%m-%d %H:%M:%S.%f')}] Incoming request: {request.method} {request.url}\n")
#         f.write(f"Headers: {dict(request.headers)}\n")
#         try:
#             f.write(f"Request files (if parsed): {list(request.files.keys()) if hasattr(request, 'files') else 'Not available'}\n")
#             f.write(f"Request form (if parsed): {dict(request.form) if hasattr(request, 'form') else 'Not available'}\n")
#         except Exception as e:
#             f.write(f"Error accessing request files or form: {str(e)}\n")
#         f.flush()
#     print(f"Incoming request logged: {request.method} {request.url}")

def resample_audio(input_path, output_path, target_sr=16000):
    """
    Converts and resamples audio file to WAV format at the target sample rate using ffmpeg.
    Forces conversion to ensure format compatibility, regardless of input format.
    Requires ffmpeg.
    """
    try:
        # Always use ffmpeg to convert to WAV format to ensure compatibility
        print(f"Forcing conversion of {input_path} to WAV format at {target_sr} Hz. Outputting to {output_path}")
        
        # Default to 1 channel if detection fails
        channels_to_use = '1'
        try:
            info = sf.info(input_path)
            print(f"sf.info for {input_path}: samplerate={info.samplerate}, format='{info.format}', channels={info.channels}, subtype='{info.subtype}'")
            if hasattr(info, 'channels') and isinstance(info.channels, int) and info.channels > 0:
                channels_to_use = str(info.channels)
        except Exception as e_info:
            print(f"[WARNING] Could not get audio info with soundfile for {input_path}: {e_info}. Defaulting to 1 channel.")
        
        print(f"Using {channels_to_use} channels for output based on input info or default.")

        command = [
            'ffmpeg',
            '-i', input_path,
            '-ar', str(target_sr),
            '-ac', channels_to_use, 
            '-f', 'wav', 
            '-y', 
            output_path
        ]
        print(f"Executing ffmpeg command: {' '.join(command)}")
        # Use check=False to capture output even on failure
        result = subprocess.run(command, check=False, capture_output=True, text=True, encoding='utf-8', errors='replace')
        
        if result.returncode != 0:
            print(f"[ERROR] ffmpeg failed. Return code: {result.returncode}")
            print(f"[ERROR] ffmpeg stdout: {result.stdout}")
            print(f"[ERROR] ffmpeg stderr: {result.stderr}")
            # Clean up potentially created partial output file
            if os.path.exists(output_path):
                os.remove(output_path)
            raise Exception(f"ffmpeg execution failed for {input_path}: {result.stderr}")
        
        print(f"ffmpeg successful for {input_path}. Saved to {output_path}")
        return output_path
    except Exception as e:
        print(f"[ERROR] resample_audio failed for {input_path}: {e}")
        # Clean up potentially created partial output file
        if os.path.exists(output_path):
            os.remove(output_path)
        raise


@app.route('/transcribe', methods=['POST'])
def transcribe_audio_endpoint():
    if 'audio' not in request.files:
        print("[ERROR] 'audio' not in request.files")
        return jsonify({"error": "No audio file provided"}), 400

    audio_file = request.files['audio']
    if audio_file.filename == '':
        print("[ERROR] audio_file.filename is empty")
        return jsonify({"error": "No selected file"}), 400

    # Create a temporary directory if it doesn't exist
    temp_dir = os.path.join(os.path.dirname(__file__), 'temp_audio')
    os.makedirs(temp_dir, exist_ok=True)

    # Save the uploaded file temporarily
    original_filename = audio_file.filename
    base_name, ext = os.path.splitext(original_filename)
    if not ext:
        ext = ".wav"
    
    temp_input_filename = f"{base_name}_{os.getpid()}_{os.urandom(4).hex()}{ext}"
    temp_input_path = os.path.join(temp_dir, temp_input_filename)
    
    try:
        audio_file.save(temp_input_path)
        if not os.path.exists(temp_input_path):
            print(f"[CRITICAL ERROR] File {temp_input_path} reported as saved but does not exist on disk!")
            return jsonify({"error": "Failed to save audio file on server"}), 500
    except Exception as e_save:
        print(f"[ERROR] Exception during audio_file.save() for {temp_input_path}: {e_save}")
        return jsonify({"error": "Failed to save uploaded audio file on server", "details": str(e_save)}), 500

    path_to_transcribe = None
    try:
        temp_resampled_filename = f"resampled_{os.path.splitext(temp_input_filename)[0]}.wav"
        temp_resampled_path = os.path.join(temp_dir, temp_resampled_filename)
        path_to_transcribe = resample_audio(temp_input_path, temp_resampled_path)

        transcription = transcribe_audio(path_to_transcribe)
        print(f"Transcription successful: {transcription}")
        return jsonify({"transcription": transcription}), 200

    except Exception as e:
        print(f"[ERROR] Transcription failed: {e}")
        return jsonify({"error": "Transcription failed", "details": str(e)}), 500

    finally:
        if os.path.exists(temp_input_path):
            os.remove(temp_input_path)
        if path_to_transcribe and os.path.exists(path_to_transcribe) and path_to_transcribe != temp_input_path:
            os.remove(path_to_transcribe)

@app.route('/test', methods=['GET', 'POST'])
def test_endpoint():
    print("--- Entered test_endpoint ---")
    return jsonify({"message": "Test endpoint reached successfully"}), 200


if __name__ == '__main__':
    # TODO: Make host and port configurable
    # Use a different port than 5000 if needed to avoid conflicts
    # Consider using waitress or gunicorn for production deployment
    app.run(debug=True, host='127.0.0.1', port=5000)
