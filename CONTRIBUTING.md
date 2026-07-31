# Contributing to ZKBind

ZKBind welcomes focused contributions that improve the correctness, reproducibility, and practical value of zero-knowledge integration analysis.

## Before contributing

Read:

- `docs/project-rfc.md` for project scope and architecture;
- `docs/threat-model.md` for modeled security properties;
- `SECURITY.md` before reporting sensitive issues.

Contributions must fit the cross-layer focus of the project. General circuit-soundness checks are better suited to established circuit-analysis tools unless they directly support proof-to-application mapping.

## Development setup

Requirements:

- Rust 1.74 or newer;
- Cargo;
- rustfmt;
- Clippy.

Run the required checks:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build --release --workspace --locked
```

Run the scanner against the included fixtures:

```bash
cargo run --locked -p zkbind-cli -- scan fixtures
cargo run --locked -p zkbind-cli -- scan fixtures --format json
```

## Rule contribution requirements

A new security rule must include:

1. a stable rule identifier;
2. a written invariant and attacker model;
3. a deliberately vulnerable fixture;
4. a secure counterpart;
5. deterministic expected scanner output;
6. a regression test;
7. documented confidence and false-positive boundaries;
8. remediation guidance;
9. source and license metadata for any externally derived benchmark.

Text matches and missing keywords must not be classified as confirmed vulnerabilities. Findings require defensible source-level evidence.

## Pull requests

Keep pull requests narrow and reviewable. Describe:

- the problem being solved;
- the design and security assumptions;
- files and interfaces changed;
- tests added;
- known limitations;
- follow-up work intentionally left out.

Avoid unrelated formatting changes or dependency additions without a clear benefit.

## Code and documentation standards

- Prefer small, explicit interfaces over premature abstraction.
- Keep scanner output deterministic.
- Preserve source locations and confidence in analysis results.
- Treat malformed projects as expected input and return actionable errors.
- Avoid panics in production paths.
- Document unsupported cases rather than silently guessing.
- Use English for repository code, documentation, issues, and pull requests.

## Licensing and attribution

Contributions are accepted under Apache License 2.0. Do not submit code that cannot legally be distributed under the project license.

When using external code, fixtures, audit findings, or datasets:

- verify the source license;
- preserve required notices;
- record the pinned source commit;
- describe modifications;
- never present copied work as original implementation.
