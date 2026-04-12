# Agent Memory RS

Secure local-first memory infrastructure for AI agents, built in Rust.

## Purpose

Agent Memory RS gives AI tools like Claude, Codex, Gemini, Hermes, OpenClaw and custom agents a better way to manage local data.

Instead of scattered JSON files, hidden temp folders, or unsafe ad-hoc state, this project provides:

- secure local persistence
- typed Rust APIs
- deterministic CLI
- project onboarding flow
- namespace-based memory
- atomic writes
- future multi-agent support

## Example CLI

```bash
agentmem init
agentmem set agent/claude/current_task "Review PR"
agentmem get agent/claude/current_task
agentmem list agent/claude
```

## First Run Onboarding

```bash
agentmem init
```

Prompts:

1. Project name
2. Storage location
3. Confirm path
4. Create config
5. Ready

## Repo Layout

```text
src/
docs/
tests/
examples/
schemas/
scripts/
```

## Status

Bootstrap phase.
