# ZKBind

**Cross-layer security analysis for zero-knowledge proof integrations.**

ZKBind is an open-source security tool for the application boundary around zero-knowledge proofs. Its goal is to trace how a statement moves from a circuit or zkVM guest through public inputs, a verifier, and into the protected action performed by an application.

```text
Circuit / zkVM guest
        ↓
Public inputs / public values
        ↓
Verifier and verification key
        ↓
Solidity integration
        ↓
Protocol state and protected action
```

A cryptographically valid proof can still be unsafe when the surrounding application does not bind it to the intended chain, contract, action, user, state root, verification key, nonce, or nullifier. ZKBind is being built to detect and explain those cross-layer failures.

> **Project status:** early research prototype. The repository is public from the beginning so that design decisions, tests, limitations, and progress remain reviewable.

## Why ZKBind

Circuit-focused tools are essential, but they usually cannot determine whether an application consumes a valid proof safely. ZKBind focuses on integration risks such as:

- cross-chain, cross-contract, cross-action, and cross-user proof replay;
- missing or incorrectly scoped nullifiers;
- stale, unauthorized, or prover-selected state roots;
- public-input ordering and encoding mismatches;
- mutable verifiers, verification keys, or zkVM program identifiers;
- verified public values that are ignored by business logic;
- application-critical values that are never committed inside the proof;
- upgrade paths that silently change proof semantics.

## Current prototype

The first executable milestone provides:

- a Rust workspace and distributable CLI foundation;
- recursive Solidity source discovery;
- conservative verifier call-site inventory;
- structured terminal and JSON reports;
- shared source-location and finding models;
- CI for formatting, linting, and tests.

The prototype intentionally reports discovery evidence before claiming vulnerabilities. Cross-layer security rules will be enabled only when they have defensible static evidence and reproducible fixtures.

## Planned MVP

The first supported vertical is:

```text
Circom + SnarkJS Groth16 + Solidity + Foundry
```

The MVP roadmap includes:

1. extracting Circom public-signal names and ordering;
2. identifying generated verifier contracts and call sites;
3. mapping Solidity values into public-signal slots;
4. building a proof-binding graph;
5. implementing replay, nullifier, domain, root, and verifier-integrity rules;
6. exporting terminal, JSON, Markdown, Mermaid, and SARIF reports;
7. generating Foundry regression-test templates for selected findings.

Noir and zkVM adapters will follow after the first stable Circom/Solidity release.

## Quick start

Requirements:

- Rust toolchain with Cargo.

Run the current scanner against a project directory:

```bash
cargo run -p zkbind-cli -- scan ./path/to/project
```

JSON output:

```bash
cargo run -p zkbind-cli -- scan ./path/to/project --format json
```

Run the quality gates:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Intended output

A mature ZKBind report will connect application values to proof semantics:

```text
msg.sender
   ↓ encoding
publicSignals[2]
   ↓
Verifier.verifyProof(...)
   ↓
claimCredential(recipient)
```

Every security finding should reference source locations, graph nodes, evidence, confidence, impact, and a reproducible remediation or test strategy.

## Repository structure

```text
crates/
├── zkbind-cli/       # command-line interface and project scanning
└── zkbind-core/      # shared models, scanner primitives, and rule interfaces

docs/
├── project-rfc.md
├── threat-model.md
└── schemas/
    └── proof-binding-graph.schema.json
```

Additional adapters, fixtures, benchmarks, Foundry harnesses, and report backends will be introduced as their implementations become testable.

## Security and responsible use

ZKBind is intended for defensive research, authorized audits, development, and CI. It does not prove circuit soundness, replace a manual security review, or authorize testing against systems without explicit permission.

Please do not report speculative scanner output as a confirmed vulnerability. Validate findings against the project specification, circuit semantics, deployment configuration, and real application flow.

## Author

Created and maintained by **Volodymyr Stetsenko** as part of an independent smart-contract and zero-knowledge security research portfolio.

## License

Apache License 2.0. See [LICENSE](LICENSE).
