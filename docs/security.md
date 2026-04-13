# Security Notes

## Rust posture

Use safe Rust by default.

Unsafe code requires:

- written justification
- review
- tests
- benchmark need

## Persistence

Write temp file then rename atomically.

## Cost + Data Exposure Controls

- keep retrieval local by default to reduce remote data transfer
- send only selected chunk excerpts, never full repository snapshots
- enforce hard token budgets before any model call
- prefer explicit fallback decisions over automatic remote escalation
