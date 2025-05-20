ADR: STT Package Index Documentation



Introduction
Prologue

Created .\packages\stt\index-stt-temp.md to document the purpose, usage for the speech-to-text (STT) package in Mivis projects.




Discussion

The stt package handles speech-to-text functionality, and its documentation in index-stt-temp.md needs to clearly explain its role and usage for using VS Code with Cline. Consistent, concise this files ensure easy navigation and integration with Cline for project management.




Solution
Purpose & Usage:

Purpose: Summarizes the stt package, which provides Vietnamese speech-to-text using Faster Whisper.

```bash
# use conda for manage enviroment:
- create new environment
conda create -n whisper-cuda python=3.10 pip=24
conda activate -n whisper-cuda

# use pytorch and cuda
- install pytorch and cuda
pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu126

# use faster whisper in speechrecognition to manage speech to text:
- install package for microphone input
pip install SpeechRecognition[audio] 

- install package faster whisper
pip install SpeechRecognition[faster-whisper]

- clone faster whisper model from PhoWhisper-ct2-FasterWhisper
git clone https://huggingface.co/quocphu/PhoWhisper-ct2-FasterWhisper

- install package for model
pip install transformers ctranslate2 faster-whisper

- other necessary pckages
pip install soundfile
pip install ffmpeg
```

Usage: Run the provided script (stt.py) with an audio file path to transcribe Vietnamese audio. Requires Conda environment setup.

# sample usesage

```python
import speech_recognition as sr
import sys

def transcribe_audio(audio_path):
    recognizer = sr.Recognizer()
    with sr.AudioFile(audio_path) as source:
        audio = recognizer.record(source)
        try:
            result = recognizer.recognize_faster_whisper(audio, language="vi", model="packages\stt\models\PhoWhisper-ct2-FasterWhisper\PhoWhisper-base-ct2-fasterWhisper")
            return result
        except sr.UnknownValueError:
            return "Không nhận diện được"
        except sr.RequestError as e:
            return f"Lỗi: {e}"

if __name__ == "__main__":
    if len(sys.argv) > 1:
        audio_path = sys.argv[1]
        print(transcribe_audio(audio_path))
    else:
        print("Vui lòng cung cấp đường dẫn file audio, ví dụ: python stt.py 'D:/path/to/sample.wav'")
```



Consequences

Pros: Updated index-stt-temp.md provides clear guidance for using the stt package. Links improve navigation across packages in VS Code via Cline.

Cons: Manual updates needed for changes in stt package or dependencies.

Status: File created, compatible with Cline for development.