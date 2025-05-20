ADR: TTS Package Index Documentation



Introduction
Prologue

Created .\packages\stt\index-tts-temp.md to document the purpose, usage for the text-to-speech (TTS) package in Mivis projects.




Discussion

The stt package handles text-to-speech functionality, and its documentation in index-tts-temp.md needs to clearly explain its role and usage for using VS Code with Cline. Consistent, concise this files ensure easy navigation and integration with Cline for project management.




Solution
Purpose & Usage:

Purpose: Summarizes the tts package, which provides Vietnamese text-to-speech using VietTTS.

# install docker on specific location
```bash
start /w "" "Docker Desktop Installer.exe" install -accept-license --installation-dir="D:\Docker\Docker" --wsl-default-data-root="D:\Docker\wsl" --windows-containers-default-data-root="D:\\Docker"
```

# use docker for manage model
1. Install [Docker](https://docs.docker.com/get-docker/), [NVIDIA Driver](https://www.nvidia.com/download/index.aspx), [NVIDIA Container Toolkit](https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/install-guide.html), and [CUDA](https://developer.nvidia.com/cuda-downloads).

2. Run the following commands:
```bash
git clone https://github.com/dangvansam/viet-tts.git
cd viet-tts

# Build docker images
docker compose build

# Run with docker-compose - will create server at: http://localhost:8298
docker compose up -d

# Or run with docker run - will create server at: http://localhost:8298
docker run -itd --gpus all -p 8298:8298 -v ./pretrained-models:/app/pretrained-models  --name viet-tts-service viet-tts:latest viettts server --host 0.0.0.0 --port 8298
```

Usage: Run the script with an input text to transcribe Vietnamese audio. More project information can find in packages\tts\models\viet-tts\README.md

# for docker usage

```bash
docker start viet-tts-service
docker stop viet-tts-service
```

# sample usesage

```python
from pathlib import Path
from openai import OpenAI

client = OpenAI()

output_file_path = Path(__file__).parent / "speech.wav"

with client.audio.speech.with_streaming_response.create(
  model='tts-1',
  voice='cdteam',
  input='Xin chào Việt Nam.',
  speed=1.0,
  response_format='wav'
) as response:
  response.stream_to_file('a.wav')
```



Consequences

Pros: Updated index-tts-temp.md provides clear guidance for using the tts package. Links improve navigation across packages in VS Code via Cline.

Cons: Manual updates needed for changes in tts package or dependencies.

Status: Created updated, compatible with Cline for development.