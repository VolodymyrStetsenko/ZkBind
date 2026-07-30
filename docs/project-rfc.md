# ZKBind Project RFC

**Status:** Accepted for implementation  
**Author:** Volodymyr Stetsenko  
**Date:** 2026-07-30

## 1. Decision

ZKBind will be an open-source cross-layer security analyzer for zero-knowledge proof integrations.

The initial release will support the following vertical:

```text
Circom / SnarkJS Groth16
        ↓
Generated Solidity verifier
        ↓
Application contract
        ↓
Foundry validation
```

The project will not begin as a general-purpose circuit auditor. Existing circuit analyzers already focus on constraint correctness. ZKBind will focus on the weaker and less automated boundary where applications interpret and consume valid proofs.

## 2. Problem statement

A verifier answers a narrow question: whether a proof is valid for public inputs under a particular verification key. It does not automatically guarantee that the surrounding application checks the intended context.

A valid proof may still be unsafe when:

- it is reusable on another chain, deployment, or function;
- a copied transaction can redirect the protected result to another recipient;
- a nullifier is absent, incorrectly scoped, checked too late, or never persisted;
- an arbitrary or stale Merkle root is accepted;
- public inputs are reordered, truncated, field-reduced, packed, or decoded inconsistently;
- the verifier or verification key can be replaced;
- a verified value is ignored;
- a security-critical application value is not committed by the proof;
- an upgrade changes proof semantics while old proofs remain valid.

These failures require a model that spans circuit metadata, verifier parameters, Solidity data flow, protocol state, and the protected business action.

## 3. Product goal

A developer or auditor should eventually be able to run:

```bash
zkbind scan .
```

and receive:

- an inventory of verifier contracts and call sites;
- public-signal names, indices, and encodings;
- a Proof Binding Graph;
- findings with source evidence and confidence levels;
- terminal, JSON, Markdown, Mermaid, and SARIF output;
- optional Foundry regression-test templates.

## 4. Design principles

### Evidence before conclusions

ZKBind must distinguish direct evidence, inferred structure, review candidates, and confirmed security violations. Discovery output such as `ZKB000` is informational and must not be reported as a vulnerability.

### Low-noise security rules

A missing value is not automatically a bug. Every rule must define when the binding is required, what evidence supports the finding, and which project-specific assumptions still need confirmation.

### Reproducibility

Every released rule requires:

1. a written invariant;
2. a vulnerable fixture;
3. a secure counterpart;
4. deterministic expected output;
5. false-positive boundaries;
6. a regression test.

### Honest scope

ZKBind will publish explicit non-goals and confidence levels. It will not claim to prove circuit soundness or replace a manual audit.

### Adapter-based architecture

Circuit languages, proving systems, verifier formats, and application platforms will be added through adapters rather than hard-coded into one parser.

## 5. Rule families

### Discovery

- `ZKB000`: likely verifier call site discovered.

### Domain binding

- `ZKB001`: proof is not bound to an explicit chain domain;
- `ZKB002`: proof is not bound to a contract or application domain;
- `ZKB003`: proof is reusable across protected actions;
- `ZKB004`: proof is not bound to the intended subject, sender, or recipient.

### Replay and nullifiers

- `ZKB101`: no replay protection is applied after verification;
- `ZKB102`: nullifier is checked but not persisted;
- `ZKB103`: nullifier scope omits a required action or domain;
- `ZKB104`: verification and replay-state mutation are ordered unsafely.

### State commitments

- `ZKB201`: root or state commitment lacks authorization or freshness validation;
- `ZKB202`: verified state differs from the state consumed by the action;
- `ZKB203`: prover-controlled state commitment is accepted without authorization.

### Verifier integrity

- `ZKB301`: verifier is mutable without an explicit protected trust boundary;
- `ZKB302`: verification key, circuit identifier, or zkVM program image is not pinned;
- `ZKB303`: upgradeability can silently change proof semantics;
- `ZKB304`: public-input count or ordering differs across layers.

### Proven-value consumption

- `ZKB401`: verified public value is ignored;
- `ZKB402`: application-critical value is not proven;
- `ZKB403`: encoding, casting, packing, or field conversion is inconsistent;
- `ZKB404`: protected action depends on uncommitted external data.

## 6. Architecture

```text
crates/
├── zkbind-cli/          command-line entry point
├── zkbind-core/         shared models and scanning primitives
├── zkbind-circom/       planned Circom and SnarkJS metadata adapter
├── zkbind-solidity/     planned Solidity AST and data-flow adapter
├── zkbind-graph/        planned Proof Binding Graph construction
└── zkbind-report/       planned report backends

fixtures/
├── vulnerable/
└── secure/

docs/
├── project-rfc.md
├── threat-model.md
└── schemas/
```

The current prototype keeps the executable foundation deliberately small. Crates will be separated when their interfaces and tests justify the split.

## 7. Proof Binding Graph

The central representation connects semantic values across layers:

```text
application value
        ↓ encoding
public-signal slot
        ↓ verifier parameter
verification result
        ↓ control-flow condition
protected action and state mutation
```

Graph nodes must preserve source locations and semantic types. Graph edges must include confidence and evidence. Findings should reference graph nodes instead of relying only on text matching.

The machine-readable schema is located at:

```text
docs/schemas/proof-binding-graph.schema.json
```

## 8. Implementation roadmap

### Milestone 0 — Foundation

- project RFC and threat model;
- Rust workspace and CLI;
- conservative Solidity verifier-call discovery;
- structured human and JSON reports;
- vulnerable/secure fixture pair;
- CI for format, lint, tests, and a scanner smoke test.

### Milestone 1 — Circom/Solidity mapping

- parse `.sym`, verification-key, and generated-verifier metadata;
- extract public-signal count and ordering;
- identify verifier interfaces, implementations, and call sites;
- map Solidity expressions into public-signal slots;
- emit a first Proof Binding Graph.

### Milestone 2 — Security rules

- implement domain, recipient, replay, nullifier, root, and verifier-integrity rules;
- add at least ten vulnerable/secure fixture pairs;
- support confidence-aware reporting;
- export SARIF and Mermaid.

### Milestone 3 — Foundry validation

- generate regression-test skeletons for supported findings;
- ship a reusable proof replay and front-running test harness;
- validate against pinned open-source projects;
- publish an independent integration-security assessment.

### Milestone 4 — Additional ecosystems

- Noir and UltraHonk adapter;
- one zkVM adapter based on stable ecosystem demand;
- program-image and public-journal binding analysis.

## 9. Non-goals for v0.1

ZKBind v0.1 will not claim to:

- prove the soundness of a circuit;
- verify the cryptographic security of a proving system;
- replace manual specification review;
- support every ZK language or verifier;
- exploit live deployments;
- classify every absent context value as a vulnerability;
- infer unavailable deployment or governance assumptions.

## 10. Open-source and authorship policy

External code may be used only when its license permits it. License notices and attribution must be preserved. Public papers, audits, and tools may inform independently implemented designs, but code will not be copied blindly or represented as original work.

Every benchmark imported from another project must record its source, pinned commit, license, expected behavior, and any modifications.

## 11. Success criteria

The first meaningful public release should:

- install and run through documented commands;
- analyze at least five complete Circom/Solidity repositories;
- implement at least ten defensible cross-layer rules;
- generate a Proof Binding Graph and SARIF report;
- maintain vulnerable/secure regression fixtures;
- publish precision, limitations, and false-positive data;
- produce at least one accepted upstream issue, fix, or integration improvement.
