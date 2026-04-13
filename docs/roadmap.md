# Roadmap

## Phase 1
- docs
- CLI init
- local store
- baseline index schema (forward + inverted)

## Phase 2
- locking
- journaling as write path
- incremental index update pipeline
- `index query` with hard token budgets

## Phase 3
- multi-agent daemon with local IPC
- VS Code integration hooks for Codex/Claude
- retrieval confidence gates before LLM fallback

## Phase 4

- quality + cost telemetry
- cache hit optimization
- compaction and large-repo performance tuning

## Success Metrics

- >= 70% of coding questions answered from local index only
- >= 50% reduction in remote tokens per task
- p95 local query latency under 100 ms for warm index
