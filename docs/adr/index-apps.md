ADR: Apps Index Documentation



Introduction
Prologue

Created index-[package name].md files in folder assistant and packages to summarize work done and provide quick references for Cline.



Discussion

Managing multiple packages (assistant, stt, tts,...) in Mivis project requires clear documentation to track purpose and usage. Without summaries, navigating and understanding package functionality is messy. This files offer a lightweight, Cline-friendly way to document and link resources.



Solution

Create Index Files: Added index-[name].md in each folder with two sections:
Purpose: Explains what the file is for
Usage: How to use it.




File Locations:

.\apps\assistant\index-assistant-temp.md
.\packages\stt\index-stt-temp.md
.\packages\tts\index-tts-temp.md



Consequences

Pros: Clear, concise documentation for each package. Easy to access via Cline in VS Code.
Cons: Requires updates when packages change.
Status: Files created, ready for Cline integration and team use.

Notes for Cline
Update content in file locations after each task.