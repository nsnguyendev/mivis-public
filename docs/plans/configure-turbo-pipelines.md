# Plan: Configure Turborepo Pipelines (turbo.json)

**Date:** 29/04/2025
**Sprint:** 1

## Goal
Configure the `turbo.json` file to define build, development, test, and lint pipelines for the Mivis monorepo, enabling efficient task execution and caching.

## Tasks

1.  **Define Base Pipeline Configuration:**
    *   Set global dependencies (e.g., `.env` files).
    *   Define base pipeline settings (e.g., `dependsOn`, `outputs`).

2.  **Configure `build` Pipeline:**
    *   Target: Build all apps and packages.
    *   Dependencies: Depends on the `build` task of internal package dependencies (`^build`).
    *   Outputs: Cache `dist/**`, `.next/**`, `build/**` directories.

3.  **Configure `dev` Pipeline:**
    *   Target: Run development servers for apps (e.g., Tauri app).
    *   Cache: Disable caching (`"cache": false`).
    *   Persistence: Mark as persistent (`"persistent": true`).

4.  **Configure `test` Pipeline:**
    *   Target: Run tests (Jest) for all packages and apps.
    *   Dependencies: Depends on the `build` task of the package itself.
    *   Outputs: Cache `coverage/**`.

5.  **Configure `lint` Pipeline:**
    *   Target: Run ESLint across the codebase.
    *   Dependencies: Depends on the `build` task of internal package dependencies (`^build`).

## Implementation Steps (ACT MODE)

1.  Use `write_to_file` to create this plan file (`/docs/plans/configure-turbo-pipelines.md`).
2.  Use `read_file` to get the current content of `turbo.json`.
3.  Use `write_to_file` to update `turbo.json` with the proposed configuration.
4.  Update `memory-bank/progress.md` to mark this task as complete.
5.  Use `attempt_completion`.

## Proposed `turbo.json` Content:

```json
{
  "$schema": "https://turbo.build/schema.json",
  "globalDependencies": ["**/.env.*local"],
  "pipeline": {
    "build": {
      "dependsOn": ["^build"],
      "outputs": ["dist/**", ".next/**", "!.next/cache/**", "build/**"]
    },
    "lint": {
       "dependsOn": ["^build"]
    },
    "dev": {
      "cache": false,
      "persistent": true
    },
    "test": {
      "dependsOn": ["build"],
      "outputs": ["coverage/**"]
    }
  }
}
```

## Verification (Manual by User or in subsequent steps)
*   Run `pnpm turbo run build`
*   Run `pnpm turbo run test`
*   Run `pnpm turbo run lint`
