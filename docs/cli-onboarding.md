# CLI Onboarding

Command:

```bash
agentmem init
```

Flow:

- ask project name
- ask storage path
- ask whether to enable local code indexing
- ask index root (default: current workspace)
- ask auto-update mode (manual, git-based, file-watch)
- confirm resolved location
- create config
- print next commands

Recommended next commands:

```bash
agentmem index build
agentmem index stats
```
