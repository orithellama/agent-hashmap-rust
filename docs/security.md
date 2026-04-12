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
