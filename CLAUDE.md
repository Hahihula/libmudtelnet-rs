# libmudtelnet Development Guide (AI Assistant Edition)

1. TL;DR for AI Assistants
- Goal: Maintain a simple, robust, idiomatic Rust Telnet (RFC 854) parser with strong real-world compatibility.
- Do:
  - Keep changes small and focused.
  - Favor clarity over cleverness.
  - Preserve behavior/API compatibility with libtelnet-rs unless explicitly justified (see §5).
  - Optimize only hot paths, measured by benchmarks (see §6, §8).
  - Add tests for new behaviors; fuzz regularly (see §8).
- Don't:
  - Over-engineer abstractions or anticipate future protocols (YAGNI).
  - Introduce panics for malformed data—be defensive and lenient.
  - Allocate in hot paths unless unavoidable and measured.
- Workflow: Implement → test (unit/compat/fuzz) → bench → clippy/fmt → PR (see §9–§12).

2. Purpose and Scope
- What this is: A low-level, event-driven Telnet protocol library for MUD clients that implements RFC 854 and provides robust parsing with minimal allocations.
- What this is not:
  - Higher-level protocols (GMCP, MSDP, MCCP) or ANSI color parsing.
  - Connection/TCP management, UI, scripting, session management.
- Design priorities: Simplicity, robustness, compatibility, performance in hot paths, idiomatic Rust.

3. Non-Negotiable Engineering Principles
- Simple: Prefer the clearest working solution; avoid cleverness.
- Robust: Handle malformed input defensively; never panic on input.
- Idiomatic Rust: Follow conventions and leverage types; keep public APIs clean and documented.
- No Overengineering: Build for current needs; avoid speculative abstractions.
- Test-Driven Behavior: Every change comes with tests to lock behavior (see §8).
- Compatibility-Aware: Behavior/API must remain consistent unless a justified policy update is made (see §5).
- Performance-Responsible: Optimize only when measured and justified (see §6).

4. Architecture and Protocol Coverage
- High-level flow:
  - Client application reads/writes TCP bytes.
  - libmudtelnet parses incoming bytes and emits events for the client.
- Core modules:
  - Parser (src/lib.rs): Stateful, event-driven parsing; minimal allocations using bytes::Bytes/BytesMut.
  - Events (src/events.rs): TelnetEvents enum; Data, Negotiation (WILL/WONT/DO/DONT), Subnegotiation, etc.
  - Protocol (src/telnet.rs): Command constants (IAC/WILL/WONT/DO/DONT/SB/SE), option codes.
  - Compatibility (src/compatibility.rs): Option support table and negotiation state.
- Protocol coverage:
  - IAC (255) handling, including proper doubling for data.
  - WILL/WONT, DO/DONT negotiations for option codes 0–255.
  - Subnegotiation: SB <option> <data> SE, with correct IAC escaping in data.
- Edge cases handled:
  - Truncated subnegotiations (e.g., IAC SB IAC SE).
  - Unescaped SE bytes appearing in data.
  - Multiple IAC escaping sequences (IAC IAC IAC IAC).
  - Non-standard server quirks and partial/incomplete sequences.
- For performance/memory rules, see §6. For compatibility guarantees, see §5.

5. Compatibility Policy
- Standards compliance: RFC 854 is the ground truth for base protocol behavior.
- Reference implementation: Maintain behavior/API compatibility with libtelnet-rs where feasible and beneficial to users.
- Behavior guarantees:
  - Event semantics are stable; changes must be backward compatible or guarded behind explicit feature flags.
  - Negotiation state management must not regress against the compatibility test suite.
  - Malformed inputs are handled gracefully without crashes or UB.
- API stability:
  - Avoid breaking public APIs; if necessary, follow semver and deprecate with clear migration notes.
- Test gating:
  - All changes must pass the compatibility tests (see §8).
  - Document any intentional deviations from libtelnet-rs with rationale in CHANGELOG and tests.

6. Performance and Memory Policy
- Goals:
  - Zero or minimal allocations in hot parsing paths.
  - Use bytes::Bytes/BytesMut to avoid copies and enable zero-copy slices where possible.
  - Avoid intermediate String conversions; operate on &[u8] slices.
- Memory management:
  - Reuse buffers; allow growth when needed but avoid thrashing.
  - Keep parser state minimal; no unnecessary heap allocations in tight loops.
  - Maintain optional no_std compatibility (no default features).
- Measurement and thresholds:
  - Benchmark critical paths before and after changes (see §8).
  - No performance regression beyond noise (agree on threshold ~3–5%) without explicit justification.
- Optimization workflow:
  - Profile → hypothesize → implement → benchmark → keep or revert.
  - If an optimization increases complexity, ensure the gain is material and documented.

7. Usage Quickstart
- Add dependency (example Cargo.toml entry):
  - libmudtelnet = { path = "../libmudtelnet" }
- Basic parser loop:
  - use libmudtelnet::{Parser, events::TelnetEvents};
  - let mut parser = Parser::new();
  - let events = parser.receive(&socket_bytes);
  - for ev in events { match ev { TelnetEvents::DataReceive(data) => { /* use data */ }, TelnetEvents::Negotiation(n) => { /* handle */ }, _ => {} } }
- Sending:
  - let bytes = parser.send_text("user command\r\n");
  - let reply = parser.negotiate(&some_negotiation_event);
- See §4 for protocol coverage and event semantics.

8. Testing and Quality Gates
- Test types and locations:
  - Unit tests (tests/): Parsing, IAC escaping/unescaping, negotiation state, edge cases.
  - Property-based testing: Round-trip invariants, escaping invariants, event/data partitioning.
  - Fuzzing (fuzz/): parser_receive, parser_escape; ensure no panics/UB on random inputs.
  - Compatibility tests (compat/): Behavior parity with libtelnet-rs on shared cases.
  - Benchmarks (compat/bench or dedicated benches): Detect regressions in hot paths.
- Canonical commands:
  - Build/test:
    - cargo build
    - cargo test
    - cargo test --release
    - cargo test --no-default-features
  - Lint/format:
    - cargo fmt
    - cargo clippy -- -D warnings
  - Fuzz (requires cargo-fuzz):
    - cargo install cargo-fuzz
    - cd fuzz && cargo fuzz run parser_receive
    - cd fuzz && cargo fuzz run parser_escape
    - cargo fuzz cmin parser_receive
  - Bench:
    - cd compat && cargo bench
- Quality gates for merging:
  - All tests (unit/property/compat) pass on debug and release.
  - No new panics on fuzz targets within reasonable runs (e.g., 60–120s).
  - No significant benchmark regressions (>5%) without documented justification.
  - clippy passes with -D warnings; code is formatted.
  - Public API changes are documented and reviewed for compatibility (see §5).

9. Developer Workflow
- Day-to-day:
  - Implement small, focused changes.
  - Write/extend tests first or alongside implementation (see §8).
  - Run cargo test; fix regressions early.
  - Fuzz locally on changed areas; minimize failing inputs if found.
  - Benchmark if your changes touch parsing hot paths.
- Debugging tips:
  - Add targeted logging behind a feature flag if needed; remove before merging or keep behind the flag.
  - Reproduce fuzz failures with minimized cases (cargo fuzz cmin ...).
- Adding protocol behaviors:
  - Keep behavior aligned with RFC 854; extend events or options conservatively.
  - Update compatibility tables and tests together (see §5, §8).
- For commands and acceptance criteria, refer to §8.

10. Contribution Standards
- Conventional Commits:
  - feat:, fix:, docs:, refactor:, test:, perf:, chore:
- Code style:
  - cargo fmt before commit; cargo clippy -- -D warnings must pass.
  - Document public APIs; prefer clear names and small modules.
- Naming conventions:
  - Modules/files: snake_case, singular (e.g., protocol.rs, state_machine.rs).
  - Types: PascalCase (e.g., TelnetEvents, ConnectionState).
  - Functions/vars: snake_case (e.g., parse_message, connection_count).
- Commit hygiene:
  - Keep commits atomic; no git commit --amend in shared history.
  - Reference issues when applicable.
- Reviews:
  - Discuss trade-offs explicitly; justify complexity with measured benefit.

11. References
- Specifications:
  - RFC 854 – Telnet Protocol
  - RFC 855 – Telnet Option Specifications
  - RFC 1073 – NAWS (Window Size)
  - RFC 1091 – Terminal Type
- Libraries/Docs:
  - libtelnet-rs (reference behavior)
  - bytes crate docs (Bytes/BytesMut)
  - Rust Fuzz Book
- Tools:
  - Wireshark (protocol byte-level inspection)
  - Real MUD servers (compatibility validation)

12. Pre-PR Checklist
- Behavior & API
  - [ ] Changes align with RFC 854 and Compatibility Policy (§5).
  - [ ] No breaking public API changes, or semver/deprecation plan documented.
- Tests & Fuzz
  - [ ] Unit/integration tests added/updated for all new behavior.
  - [ ] Fuzz targets run without new crashes; minimized cases added as tests if relevant.
  - [ ] Compatibility tests pass; deviations documented and justified.
- Performance
  - [ ] Benchmarks run; no significant regressions (>5%) unless justified.
  - [ ] No unnecessary allocations added to hot paths; Bytes/BytesMut used where appropriate (§6).
- Quality
  - [ ] cargo fmt run; cargo clippy -- -D warnings passes.
  - [ ] Public APIs and behaviors documented; changelog notes (if needed).
- Ready to Merge
  - [ ] Commit messages follow Conventional Commits and are atomic.
  - [ ] Trade-offs and rationale captured in PR description.