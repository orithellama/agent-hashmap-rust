# API

## Core Store API

```rust
Store::open(config)
store.set(key, value)
store.get(key)
store.delete(key)
store.list(prefix)
store.flush()
```

## Index API (Cost-First)

```rust
index.build(root) -> IndexBuildReport
index.update(changed_files) -> IndexUpdateReport
index.query(query, top_k, token_budget) -> QueryResult
index.get_chunk(chunk_id) -> Chunk
index.stats() -> IndexStats
```

## Suggested CLI Surface

```bash
agentmem index build
agentmem index update --changed-from-git
agentmem index query "where is auth middleware" --top-k 8 --token-budget 4000
agentmem index stats
```

## Query Contract

`index.query(...)` should return:

- ranked chunk IDs
- file paths + line ranges
- confidence score
- estimated token cost for selected chunks
- `fallback_required` flag

If confidence is high and token budget is respected, clients should answer from
retrieved local chunks and skip remote model calls.

## Cost Controls

- hard `token_budget` per query
- hard `top_k` cap
- local cache key = `project_hash + commit_hash + normalized_query`
- return deterministic empty/no-match instead of triggering automatic LLM calls
