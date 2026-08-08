# ZKBind

[![CI](https://github.com/VolodymyrStetsenko/ZkBind/actions/workflows/ci.yml/badge.svg)](https://github.com/VolodymyrStetsenko/ZkBind/actions/workflows/ci.yml)
[![Version](https://img.shields.io/badge/version-0.1.0-111111)](Cargo.toml)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

**Cross-layer security analysis for zero-knowledge proof integrations.**

ZKBind analyzes how zero-knowledge proofs are connected to application logic. It traces the boundary between a circuit or zkVM program, public values, a verifier, and the state-changing action protected by the proof.

```text
Circuit or zkVM program
        ↓
Public inputs / public values
        ↓
Verifier and verification key
        ↓
Solidity integration
        ↓
Protocol state and protected action
```

A proof may be cryptographically valid while the surrounding application remains vulnerable. Common integration failures include proof replay, missing domain separation, incorrectly scoped nullifiers, unauthorized state roots, public-input ordering errors, mutable verifiers, and application values that are not committed by the proof.

## Current capabilities

The current implementation provides a tested foundation for Solidity integration analysis:

- recursively discovers Solidity source files;
- ignores common generated and dependency directories;
- locates likely `verifyProof` and `verify_proof` call sites;
- reports exact file, line, and column locations;
- returns findings in deterministic source-path order;
- uses project-relative finding paths for portable reports;
- produces human-readable and JSON output;
- uses stable rule identifiers, severity, and confidence fields;
- includes vulnerable and secured integration fixtures;
- runs formatting, linting, unit tests, release builds, and scanner smoke tests in CI.

The current rule `ZKB000` is an informational discovery record. It identifies a likely verifier call site but does not classify it as a vulnerability.

ZKBind is a focused research foundation, not an automated security verdict. Cross-layer proof integration review remains a source-level reasoning task. For a defined protocol engagement, see [Working With Me](https://github.com/VolodymyrStetsenko/VolodymyrStetsenko/blob/main/WORK_WITH_ME.md).

## Installation

Requirements:

- Rust 1.74 or newer;
- Cargo.

Build an optimized binary:

```bash
git clone https://github.com/VolodymyrStetsenko/ZkBind.git
cd ZkBind
cargo build --release --locked
```

The binary is created at:

```text
target/release/zkbind
```

Install it into Cargo's binary directory:

```bash
cargo install --path crates/zkbind-cli --locked
```

## Usage

Scan a project:

```bash
zkbind scan ./path/to/project
```

Scan the current directory:

```bash
zkbind scan
```

Produce JSON output:

```bash
zkbind scan ./path/to/project --format json
```

Display help or version information:

```bash
zkbind --help
zkbind --version
```

## Example output

```text
ZKBind scan
root: /workspace/protocol
Solidity files: 14
verifier call sites: 2
src/Claim.sol:61:17  ZKB000  Verifier call site discovered
src/Vote.sol:84:21   ZKB000  Verifier call site discovered
```

JSON reports include a schema version, scan root, number of Solidity files, findings, severity, confidence, messages, and source locations.

## Analysis target

ZKBind is designed to answer questions that circuit-only tools cannot resolve on their own:

- Is a proof bound to the correct chain, contract, action, and recipient?
- Can the same proof or nullifier be reused?
- Does the application authorize and validate the state root used by the proof?
- Are public inputs constructed in the correct order and encoding?
- Is the intended verifier, verification key, or program identifier pinned?
- Are security-critical application values committed inside the proof?
- Are verified values actually consumed by the protected action?

## Supported stack

The first complete analysis path targets:

```text
Circom + SnarkJS Groth16 + Solidity + Foundry
```

Support for Noir and zkVM integrations will be added through separate adapters after the Circom/Solidity analysis path is stable.

## Project structure

```text
crates/
├── zkbind-cli/       # command-line interface
└── zkbind-core/      # source discovery, findings, and report models

fixtures/
├── vulnerable/      # intentionally unsafe integrations
└── secure/          # corrected counterparts

docs/
├── project-rfc.md
├── threat-model.md
└── schemas/
    └── proof-binding-graph.schema.json
```

The architecture and implementation milestones are documented in [`docs/project-rfc.md`](docs/project-rfc.md). Security assumptions and analysis boundaries are documented in [`docs/threat-model.md`](docs/threat-model.md).

## Current limitations

ZKBind currently performs verifier call-site discovery. It does not yet:

- parse Solidity AST or control-flow graphs;
- map expressions into public-signal indices;
- parse Circom `.sym`, R1CS, or verification-key metadata;
- detect replay, nullifier, domain-binding, or root-validation vulnerabilities automatically;
- prove circuit soundness;
- replace manual security review.

Scanner output must be validated against the protocol specification, circuit semantics, deployment configuration, and application flow.

## Development

Run all local checks:

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

Contribution requirements are documented in [`CONTRIBUTING.md`](CONTRIBUTING.md). Security reports should follow [`SECURITY.md`](SECURITY.md).

## Maintainer

**Volodymyr Stetsenko**

## License

Apache License 2.0. See [`LICENSE`](LICENSE).
