* **Project Brief: Desktop Assistant - Mivis**



* **Overview**
Desktop Assistant is a personal AI-powered app for Windows 11, built to:

Handle Vietnamese voice/text input using Whisper (STT) and VietTTS (TTS).
Control PC operations (open apps, read/write files, monitor CPU/RAM) with Node.js.
Search web and summarize results with Tavily API (TypeScript SDK).
Interact with Gmail, Discord, Spotify via MCP - Model Context Protocol (MCP Server, Client, Agent).
Provide natural language responses using LLM API.




* **Goals**

Build a lightweight, secure desktop app.
Support Vietnamese voice/text interaction with STT, TTS.
Automate email, messaging, and music playback with rule-based and LLM.
Achieve MVP in 8 weeks (4 Sprints, starting 28/04/2025).
Use Cline to automate coding, testing, and documentation.




* **Project Technology Stack**

**Project Wrapper (Turborepo, Tauri)**
Manage project structure conveniently.
Keep project structure clean and easy to maintain.
friendly to handle computer operations.

**Frontend (SvelteKit, SvelteKit AI Chatbot template by Vercel)**
Process chatbox interface, display results.
For user experience, use UI components: 
Notifications, Tooltips, Checkboxes, Progress bars, Sliders, Tabs, Pagination, Search field
Chat logic (parse input, call LLM API, format response).
Auth (OAuth for MCP Server, use MCP Client).
Web search (Tavily API, use TypeScript SDK).
MCP Client and MCP Agent (parse Vietnamese commands, send requests to MCP Server).

**IPC Wrapper (Svelte framework to Tauri IPC (Inter-Process Communication))**
Convert Svelte framework request (frontend) into Tauri IPC command to communicate with Rust command (backend).
And vice versa.
Use Synchronous (request/response) and Asynchronous (request/async response) 
as appropriate for communication sessions between components.

**Backend (Rust/Node.js)**
Handle STT (Whisper) and TTS (VietTTS).
Control PC (open app, read/write file, monitor CPU/RAM).
Other logic code need to handle at backend.

**MCP Server**
Handle routes, OAuth, call third party APIs (Gmail, Discord, Spotify).
Save logs to Cloudflare KV.




* **MCP Integration**

**Overview:**
Use Anthropic MCP for Gmail, Discord, Spotify APIs, implemented via MCP Server (/packages/mcp-server), MCP Client (/packages/mcp-client), and MCP Agent (/packages/agent) within monorepo.
MCP Server runs on Cloudflare Workers, uses Hono framework for API routes (/mcp/spotify, /mcp/gmail, /mcp/discord).
MCP Client uses TypeScript SDK for JSON/SSE requests.
MCP Agent uses TypeScript + Grok 3 API (Sprint 3+) for input parsing and service dispatching.
Follow guide to create MCP log docs\plans\logging-plan.md

**MCP Server:**
Deploy in /packages/mcp-server, Cloudflare Workers, Hono routes.
Handles OAuth (routes /auth, /callback), stores tokens in Cloudflare KV.
Supports Spotify (play/pause, scope user-modify-playback-state), Gmail (send email), Discord (send message).
Log errors to /logs/mcp.log (Cloudflare storage), exclude sensitive data.
Config-driven (JSON/YAML) for adding services.

**MCP Client:**
Implement in /packages/mcp-client, TypeScript SDK, JSON/SSE requests.
Sends commands (e.g., { method: 'play_song', params: { service: 'spotify', track_uri: '...' } }) to MCP Server.
Manages tokens (cache in-memory Map, refresh via /auth), retries (3 attempts, exponential backoff).
Logs errors to /logs/mcp.log, no sensitive data.
Supports real-time SSE (e.g., Spotify playback state).

**MCP Agent:**
Implement in /packages/agent, TypeScript + Grok 3 API (Sprint 3 for NLP).
Parses Vietnamese input (regex in Sprint 2, Grok 3 in Sprint 3) from Whisper/UI.
Dispatches commands to MCP Client (e.g., Spotify, Gmail, Discord).
Manages state (tokens, device_id) via in-memory cache.
Outputs via Tauri IPC to Svelte UI or VietTTS.
Modular design (Input Processor, Service Dispatcher, State Manager, Output Handler).

**Integration:**
MCP Client calls MCP Server (e.g, https://mcp-spotify.<your-cloudflare>.workers.dev).
Agent integrates with MCP Client, Tauri IPC, and Grok 3 API.
Use .env for API keys (SPOTIFY_CLIENT_ID, GMAIL_CLIENT_ID, etc.).
Update activeContext.md after done tasks.
Example:
Agent parses “Mở bài Despacito” → MCP Client sends { method: 'play_song', params: { service: 'spotify', track_uri: '...' } } → MCP Server calls Spotify API → Response to Agent → Output to Svelte UI/VietTTS.



* **Tech Stack**

Monorepo: Managed by Turborepo, using pnpm for package management.
Frontend: Tauri app (/apps/assistant) with SvelteKit AI Chatbot template by Vercel.
Backend: Rust (Tauri IPC for native calls), Node.js (/packages/system-control) for PC control.
AI: LLM API (/packages/ai) for NLP, Tavily (/packages/web-search) for search using TypeScript SDK.
STT/TTS: Whisper (/packages/stt) and VietTTS (/packages/tts) for Vietnamese voice processing.
Integrations:
MCP for Gmail, Discord, Spotify APIs, implemented via:
MCP Server (/packages/mcp-server): Runs on Cloudflare Workers, uses Hono for routes (/mcp/spotify, /mcp/gmail, /mcp/discord), handles OAuth, logs errors to Cloudflare KV.
MCP Client (/packages/mcp-client): TypeScript SDK (@anthropic/mcp-client or custom), sends JSON/SSE requests, manages tokens, logs errors to /logs/mcp.log.
MCP Agent (/packages/agent): TypeScript + LLM API, parses Vietnamese input (regex in Sprint 2, NLP in Sprint 3), dispatches to MCP Client, outputs via Tauri IPC.
Cline: Automates coding, testing, docs, reads Memory Bank for context.



* **Dependencies**

Node.js: v22+ (system control, API clients).
pnpm: v9+ (package manager).
Rust: Latest stable (Tauri backend).
TypeScript: v5+ (all JS code).
SvelteKit AI Chatbot template by Vercel: Frontend framework (Tauri).
LLM API: NLP (e.g: xAI).
Tavily API: Web search (TypeScript SDK).
Whisper: STT model (/packages/stt/models).
VietTTS: TTS model (/packages/tts/models).
MCP Server: Hono (Cloudflare Workers), Cloudflare KV for logging.
MCP Client: TypeScript SDK, supports JSON/SSE.
MCP Agent: TypeScript, LLM API for NLP.
ESLint/Prettier: Code style (/configs).
Additional:
rotating-file-stream: Log rotation for /logs/mcp.log (Client/Agent).
sanitize: Sanitize sensitive data in logs.


* **Monorepo Structure**

/apps/assistant: Tauri app (Svelte frontend).
/docs: Plans, ADRs.
/logs: Error logs (/logs/mcp.log for Client/Agent, Cloudflare KV for Server).
/memory-bank: Context for Cline (this folder).
/packages/stt: Whisper STT.
/packages/tts: VietTTS.
/packages/ai: LLM API.
/packages/agent: MCP Agent (TypeScript, input parsing).
/packages/mcp-server: MCP Server (Cloudflare Workers, Hono, OAuth).
/packages/mcp-client: MCP Client (TypeScript SDK, JSON/SSE).
/packages/system-control: Node.js PC control.
/packages/web-search: Tavily API (TypeScript SDK).



* **Key Contacts**

Developer: Nguyên (indie maker, AI enthusiast).
GitHub: nsnguyendev/mivis.
Start Date: 28/04/2025.