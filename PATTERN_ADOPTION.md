# Pattern adoption

Declared per references/adoption-format.md; IDs resolve against
references/catalog.md in pattern-standards.

| Pattern | Status | Evidence |
|---|---|---|
| T1 | adopted | src/types.rs:6 (serde typed ticker structs parsed once at the WS boundary) |
| T16 | n/a | price ticks land in a single signal; no streaming backpressure surface |
