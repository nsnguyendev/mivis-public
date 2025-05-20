Note: This is a personal project currently under development. The setup process, especially for STT/TTS, can be complex and may not be user-friendly for non-technical users at this stage.

# 🤖 Mivis - AI Assistant Monorepo

Mivis is a personal desktop Vietnamese assistant for Windows 11, powered by AI. Built with Tauri, Turborepo, SvelteKit, Rust and xAI Grok.

In particular, this project is built with vibe coding: 
- Grok 3 for research, planning, technical consulting. 
- Gemini 2.5 pro for programming, coding maintenance, documentation.

The project also applies AI prompt techniques in remembering project information: 
- clinerule, clineignore: helps Foundation Model (FM) maintain consistent behavior.
- memory-bank:
    - projectbrief.md: helps FM maintain seamless project information. 
    - activeContext.md: helps FM remember information about working sessions.
docs/plans:
    - sprint-plan.md: remembers tasks to be done. 
    - other package instructions: helps LM remember specific functional instructions.


## 🌟 Project Features

- **Voice/Text Interface**: Faster-Whisper STT (using PhoWhisper model) for speech-to-text and VietTTS for text-to-speech.
- **AI-Powered**: xAI Grok for natural language processing and automation.
- **Lightweight**: Built with Tauri, aims for low resource usage and PC friendly.
- **Monorepo**: Organized with Turborepo for easy scaling.

## 🪄 Demo

https://github.com/user-attachments/assets/2a3f4d56-eb7b-4fc3-b243-3719f1830b9d

## 🔮 Implementation Goals

- **PC Control**: Open apps, read/write files, monitor CPU/RAM (Node.js).
- **Web Search**: Search and summarize web results (Tavily).
- **Third-Party Integration**:
    - Gmail: Read, summarize, and send emails.
    - Discord: Send/receive messages with rule-based replies.
    - Spotify: Play songs, save music preferences (rock, indie).

## ⚙️ Prerequisites

- **Node.js**: v22+ (or higher).
- **pnpm**: v9+ (or higher).
- **Rust**: Latest stable (for Tauri).
- **Windows 11**: Target platform.
- **Docker Desktop**: Installed and running (required for VietTTS).
- **Conda**: Miniconda or Anaconda (recommended for managing Python environment for STT).
- **API Keys**: xAI API Key (for Grok).

## ✅ To-Do / Future Enhancements

- [x] **Core Functionality**: 
    Voice/text -> STT (Whisper) -> AI API response -> TTS (VietTTS).
- [ ] **Reengineering core functionality**:
    Frontend: SvelteKit + Vercel AI Chatbot framework for Svelte.
    ICP Wrapper: TypeScript.
    Backend: Rust/Node.js
- [ ] **MCP Client & Agent in Frontend**
- [ ] **Web Search with Tavily**
- [ ] **PC Control in Backend**
- [ ] **User Settings**
- [ ] **UI/UX Polish**
- [ ] **Optimize STT/TTS**

## 🛠️ Installation

### A. Clone Mivis Repository & Initial Setup

1.  **Clone the repo**:
    ```bash
    git clone https://github.com/nsnguyendev/mivis.git 
    # Replace with your fork if you plan to contribute later
    cd mivis
    ```
2.  **Install dependencies**:
    ```bash
    pnpm install
    ```
3.  **Setup environment**:
    - Copy `.env.example` to `.env`:
        ```bash
        cp .env.example .env
        ```
    - Add your API keys to the `.env` file:
        ```
        XAI_API_KEY=your_xai_api_key
        ```

### B. Setup Speech-to-Text (STT - Whisper/PhoWhisper)

1.  **Create and Activate Conda Environment**:
    ```bash
    conda create -n whisper-cuda python=3.10 pip=24 -y
    conda activate whisper-cuda
    ```
2.  **Install PyTorch with CUDA support**: (Ensure your NVIDIA drivers are up-to-date and CUDA toolkit is compatible. Adjust `cu126` to your CUDA version e.g., `cu118`, `cu121` if necessary. For CPU-only, you might need a different PyTorch command.)
    ```bash
    pip install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu126
    ```
3.  **Install STT-related Python Packages**:
    ```bash
    pip install SpeechRecognition[audio] SpeechRecognition[faster-whisper] transformers ctranslate2 faster-whisper soundfile ffmpeg
    ```
4.  **Download PhoWhisper Model**:
    ```bash
    # Navigate to packages/tts from project root, create 'models' if it doesn't exist
    cd packages/stt/
    mkdir -p models 
    git clone https://huggingface.co/quocphu/PhoWhisper-ct2-FasterWhisper
    ```
    *The application expects the model at `packages/stt/models/PhoWhisper-ct2-FasterWhisper`.*

### C. Setup Text-to-Speech (TTS - VietTTS via Docker)

*Note: The VietTTS Docker setup can consume significant disk space (approx. 20GB for images and models) and system resources. A GPU is recommended for optimal performance.*

1.  **Ensure Docker Desktop is installed and running.**
2.  **(Optional but Recommended for GPU) Install latest NVIDIA Drivers, NVIDIA Container Toolkit, and a compatible CUDA Toolkit** by following their official installation guides. This is for GPU acceleration within Docker.
3.  **Clone VietTTS Repository**: (It's recommended to clone it into the Mivis project structure for organization)
    ```bash
    # Navigate to packages/tts from project root, create 'models' if it doesn't exist
    cd packages/tts
    mkdir -p models 
    cd models
    git clone https://github.com/dangvansam/viet-tts.git
    cd viet-tts 
    ```
4.  **Build VietTTS Docker Image**: (This might take some time)
    ```bash
    docker compose build
    ```
5.  **Run VietTTS Docker Container**:
    ```bash
    docker compose up -d 
    ```
    This will start the TTS service, typically accessible at `http://localhost:8298`.

### D. Run Mivis Development Server

After completing all setup steps (A, B, and C):
```bash
cd apps/assistant
pnpm tauri dev
```
This command starts the web development server and should also launch the Tauri application.

### E. Build Mivis Tauri App (for production)

To create a standalone application:
```bash
pnpm tauri build
```

## 😎 Usage

Before starting Mivis:
1.  Ensure your STT Conda environment (`whisper-cuda`) is active if running Python scripts directly. (The Tauri application's sidecar for STT should manage its environment, but this is good practice for direct script testing).
2.  Verify the VietTTS Docker container is running. You can check with `docker ps` (should list a container for `viet-tts`) or navigate to the `packages/tts/models/viet-tts` directory and run `docker compose ps`.

To interact with Mivis:
- **Voice**: Speak in Vietnamese.
- **Text**: Type in the chat box. 

## 🏗️ Project Structure

```
/mivis (project-root)
├── /apps
│   └── /assistant (Tauri app, SvelteKit frontend)
├── /packages
│   ├── /stt (Whisper STT - PhoWhisper model)
│   │   └── /models/PhoWhisper-ct2-FasterWhisper (cloned model)
│   └── /tts (VietTTS - integration via Docker service)
│       └── /models/viet-tts (cloned VietTTS source for Docker)
├── package.json
├── turbo.json
└── README.md
```

## 🛂 Contributing

This is a personal project currently under development. Contributions will be welcome in the future once the project reaches a more stable state and clear contribution guidelines are established. Thank you for your interest!

## 📄 License

Mivis is licensed under the MIT License. Feel free to use, modify, and distribute the code, just keep the copyright notice and disclaimer intact. Check the LICENSE file for details.

## 📜 Acknowledgements / Citations
- **STT (Speech-to-Text)**: This project utilizes the [Whisper](https://openai.com/research/whisper) model architecture. Specifically, it employs the [PhoWhisper](https://huggingface.co/quocphu/PhoWhisper-ct2-FasterWhisper) pre-trained model by `quocphu`, run with the [Faster Whisper](https://github.com/SYSTRAN/faster-whisper) implementation by SYSTRAN.
- **TTS (Text-to-Speech)**: Text-to-Speech functionality is provided by [VietTTS](https://github.com/dangvansam/viet-tts) developed by `dangvansam`. In this project, it is run as a Docker service. 

## 💭 Contact 
Github: [nsnguyendev](https://github.com/nsnguyendev) in 🌌
