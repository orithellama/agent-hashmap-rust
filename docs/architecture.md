# Architecture

## Layers

1. CLI layer
2. Query layer
3. Index layer
4. Store layer
5. In-memory engine
6. Config layer

## Cost-First Flow

1. User asks question in Codex/Claude.
2. Agent calls local `index.query(...)`.
3. Query layer enforces `top_k` and `token_budget`.
4. If confidence is sufficient, answer from local chunks only.
5. Only low-confidence queries may escalate to remote model context.

The default path must be "local retrieval only".

## Core Principles

- local-first
- explicit paths
- atomic writes
- typed interfaces
- no hidden side effects
- predictable query cost
- incremental indexing only

## Index Design

- Forward index: `chunk_id -> {path, line_start, line_end, hash, text_ref}`
- Inverted index: `token/symbol -> [chunk_id...]`
- Metadata index: language, file size, last modified hash

This avoids full-repo scans for every question.

## Performance Design

- daemon holds hot index in memory
- incremental update from file hashes and git diff
- append journal for updates; periodic snapshot compaction
- no full snapshot rewrite on each mutation

## Retrieval Strategy

- rank by lexical and symbol matches first
- cap results by bytes/tokens before model call
- include precise file/line references in output
- prefer deterministic "not enough evidence" over expensive fallback

## Telemetry (Local)

Track per-project:

- queries served from index only
- remote fallback rate
- average token payload sent to model
- index build/update durations

Primary KPI: reduce remote-token usage while preserving answer quality.
