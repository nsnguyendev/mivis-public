Sprint Plan for Mivis Desktop Assistant
Product Goal
Build a personal desktop assistant on Windows 11, supporting:

Voice/text interaction (Whisper/VietTTS).
PC control (open apps, read/write files).
Web search (Tavily).
Interaction with Gmail, Discord, Spotify via MCP (Cloudflare Workers, /packages/mcp-server).
Use Cline to automate coding, testing, documentation.

Kanban Board

Columns: To Do, In Progress, Done.
Properties:
Task Name (Text).
Sprint (Select: Sprint 1, Sprint 2, Sprint 3, Sprint 4).
Priority (Select: High, Medium, Low).
Story Points (Number).
Acceptance Criteria (Text).
Status (Status: To Do, In Progress, Done).
Test Status (Select: Not Tested, Passed, Failed).
Cline Usage (Text): Describe how Cline is used for the task.



Timeline View

Track progress weekly from 28/04/2025 to 20/06/2025 (8 weeks, 4 Sprints).
Milestones: Sprint 1 (09/05/2025), Sprint 2 (23/05/2025), Sprint 3 (06/06/2025), Sprint 4 (20/06/2025).

Sprint Planning
Sprint 1 (28/04/2025 - 09/05/2025) - Completed

Goal: Set up Turborepo, Tauri, STT/TTS, Cline, basic tests, and monorepo structure with Memory Bank.
Tasks:
Setup Turborepo Monorepo (3 SP, High):
Acceptance: Monorepo structure works (apps/assistant, packages/*, memory-bank/*), pnpm dev OK, ESLint/Prettier OK.
Cline Usage: Create monorepo structure, configure ESLint/Prettier.


Initialize Tauri App (3 SP, High):
Acceptance: Tauri app runs, basic Svelte UI, system tray OK.
Cline Usage: Create Tauri app, Svelte UI, test initial build.


Integrate Whisper STT (2 SP, Medium):
Acceptance: Record Vietnamese audio, display text.
Cline Usage: Integrate Whisper create unit tests.


Integrate VietTTS TTS (2 SP, Medium):
Acceptance: Convert text to speech, audio playback OK.
Cline Usage: Integrate VietTTS, create unit tests.


Setup Cline and .clinerules (2 SP, High):
Acceptance: Cline installed in VS Code, .clinerules created, Memory Bank (projectbrief.md, techContext.md, activeContext.md, progress.md) initialized, test prompt OK.
Cline Usage: Configure Cline, create .clinerules, test React component.




Cline Usage: Generate structure, configs, UI, tests, update Memory Bank.
Retrospective: [Pending notes, assume smooth setup, Cline saved time on configs/tests].

Sprint 2 (12/05/2025 - 23/05/2025)

Goal: Add AI chat box, web search, PC control, basic MCP Server and Client, integrated tests.
Tasks:
Integrate Vercel AI SDK + Grok 3 with Cline (3 SP, High):
Acceptance: Chat box runs, Grok 3 API works, .env secure, techContext.md updated.
Cline Usage: Create Svelte component, API call

 - **Integrate STT-TTS to chat box to enable voice chat:** Develop a voice chat component in the chat box using STT-TTS package implemented in Sprint 1.

Integrate Tavily API (2 SP, Medium):
Acceptance: Web search works, summary displayed, API client from Tavily SDK.
Cline Usage: Create API client,


Integrate System Control (3 SP, High):
Acceptance: Open apps, read/write JSON, display CPU/RAM, Jest tests.
Cline Usage: Create Node.js functions


Setup MCP Server on Cloudflare Workers (3 SP, High):
Acceptance: MCP Server runs in /packages/mcp-server, deployed to Cloudflare, supports Spotify (play/pause), .env secure, activeContext.md updated.
Cline Usage: Create Hono routes, configure OAuth


Setup MCP Client in Monorepo (3 SP, High):
Acceptance: MCP Client in /packages/mcp-client, connects to MCP Server, tests Spotify API, logs errors to /logs/mcp.log, techContext.md updated.
Cline Usage: Create TypeScript SDK



Setup log for MCP Server, MCP Client (2 SP, High):
Acceptance: Setup log for MCP Server (/packages/mcp-server): Hono middleware, Cloudflare KV.Setup log for MCP Client (/packages/mcp-client): File /logs/mcp.log, sanitizer.
Cline usage: Create log



Cline Usage: Generate components, API clients, tests, configs, update Memory Bank.
Retrospective: [To be filled post-Sprint, expect Cline to streamline MCP setup].

Sprint 3 (26/05/2025 - 06/06/2025)

Goal: Integrate Agent, Gmail, Discord via MCP, integrated tests.
Tasks:
Build Agent with TypeScript + Grok 3 (4 SP, High):
Acceptance: Agent in /packages/agent, parses Vietnamese input (regex), calls MCP Client, supports Spotify, .env secure, activeContext.md updated.
Cline Usage: Create TypeScript modules


Integrate Gmail API via MCP (4 SP, High):
Acceptance: Read/summarize/send email, OAuth2 OK, Jest tests, MCP Server route /mcp/gmail.
Cline Usage: Create API integration


Integrate Discord API via MCP (3 SP, Medium):
Acceptance: Read/send messages, rule-based logic OK, MCP Server route /mcp/discord, activeContext.md updated.
Cline Usage: Create rule-based logic


Create log for MCP Agent (2 SP, Medium):
Acceptance: Create log cho MCP Agent (/packages/agent): Parsing, dispatching errors.
Add log (integration tests Agent → Client → Server).
Cline usage: Create log.


Cline Usage: Generate modules, tests, configs, update Memory Bank.
Retrospective: [To be filled post-Sprint, expect Agent to streamline multi-service].

Sprint 4 (09/06/2025 - 20/06/2025)

Goal: Add Spotify API, save music preferences, integrate NLP, optimize
Tasks:
Integrate Spotify API via MCP (4 SP, High):
Acceptance: Play songs, save preferences in JSON, OAuth2 OK, MCP Server route /mcp/spotify, README.md updated.
Cline Usage: Create API integration, unit tests.


Integrate NLP with Grok 3/Gemini API into Agent (3 SP, High):
Acceptance: Agent parses natural input (Grok 3/Gemini API), supports Spotify/Gmail/Discord, techContext.md updated.
Cline Usage: Create NLP logic, integration tests.



Cline Usage: Generate integrations, tests, optimizations, update docs.
Retrospective: [To be filled post-Sprint, expect NLP and E2E to finalize MVP].


Testing Tools

Jest: Unit tests for JS/TS, Cline generates test cases.
Playwright: E2E tests for UI and flows, Cline generates scripts.
Manual Test: UI, STT/TTS, MCP, performance on Windows 11.
Coverage: nyc or jest --coverage, target 80%.


Note for Cline:
Update docs\adr\projectSummary.md, memory-bank\activeContext.md, memory-bank\progress.md if needed.
