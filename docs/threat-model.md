# ZKBind Threat Model

## Purpose

ZKBind analyzes whether an application safely consumes a zero-knowledge proof after cryptographic verification. It focuses on the semantic boundary between a circuit or zkVM program, its public values, the verifier, and the protected application action.

ZKBind does not attempt to prove the soundness of an underlying proving system or replace a complete circuit audit.

## Assets

The primary assets protected by a ZK integration include:

- authorization to execute a privileged action;
- uniqueness of a claim, vote, withdrawal, credential, or message;
- integrity and freshness of a committed state root;
- confidentiality properties promised by the application;
- correctness of public-input interpretation;
- integrity of the verifier, verification key, and program identifier;
- protocol funds, state transitions, permissions, and identities.

## Trust boundaries

```text
Untrusted prover-controlled data
        ↓
Circuit / zkVM statement
        ↓
Public values and proof bytes
        ↓
Verifier contract or verifier library
        ↓
Application validation and state mutation
        ↓
Protected action
```

ZKBind treats calldata, proof bytes, public inputs, transaction ordering, and off-chain metadata as attacker-controlled unless the application establishes a stronger guarantee.

## Adversary capabilities

The baseline adversary may:

- observe valid proofs and public inputs in a public mempool;
- copy, reorder, front-run, and replay transactions;
- choose calldata values not cryptographically committed by the proof;
- submit a proof on another chain, contract, function, or protocol deployment;
- choose stale or attacker-controlled roots when the application permits it;
- exploit inconsistent encoding, truncation, field reduction, or signal ordering;
- call public functions through contracts or relayers;
- exploit verifier mutability, proxy upgrades, weak governance, or configuration errors;
- combine individually valid components in a semantically invalid application flow.

The baseline adversary is not assumed to break a secure hash function, forge a proof for a sound circuit, recover secret witnesses from a zero-knowledge proof, or compromise trusted hardware unless a specific adapter models that threat.

## Security properties

### Domain binding

A proof used for a protected action should be bound, where required by the protocol specification, to:

- the chain or rollup domain;
- the verifying contract or application identifier;
- the action, function, or message type;
- the intended subject, sender, recipient, or beneficiary;
- a protocol version when upgrades can change proof semantics.

### Replay resistance

A one-time proof should include a correctly scoped nullifier, nonce, message identifier, or equivalent replay-control value. The application must check and persist that value atomically with the protected action.

### State commitment integrity

A state root or commitment must be authorized and sufficiently fresh for the action. A prover must not be able to select an arbitrary root that makes an otherwise invalid statement appear valid.

### Verifier integrity

The application must verify against the intended verifier, verification key, circuit identifier, or zkVM program image. Mutation and upgrade paths must preserve explicit trust and version boundaries.

### Public-value consistency

Public values must retain the same meaning, ordering, width, encoding, and field representation across:

1. the circuit or guest program;
2. proving-system metadata;
3. verifier parameters;
4. Solidity construction and decoding;
5. application business logic.

### Proven-value consumption

Values that authorize or parameterize the protected action should be committed inside the proof and checked by the application. Conversely, verified values that carry security meaning should not be silently ignored.

## Finding confidence

ZKBind separates evidence from conclusions:

- **High confidence:** direct source-level data flow or explicit invariant violation;
- **Medium confidence:** strong structural evidence requiring limited specification confirmation;
- **Low confidence:** a review candidate or missing binding that may be intentional.

Discovery records such as `ZKB000` are informational inventory and must not be presented as vulnerabilities.

## Out of scope for v0.1

The first release does not claim to detect:

- cryptographic flaws in Groth16, PLONK, STARKs, curves, pairings, or hash functions;
- general underconstrained or overconstrained circuit bugs;
- witness-generation discrepancies without adapter evidence;
- compromised ceremonies or toxic-waste retention;
- side-channel leakage in prover implementations;
- vulnerabilities requiring unavailable deployment configuration;
- live exploitation or unauthorized testing.

## Validation requirements

A rule is eligible for release only when it has:

- a written invariant and attacker model;
- at least one intentionally vulnerable fixture;
- a secure counterpart;
- deterministic expected output;
- documented false-positive boundaries;
- source-level evidence in the report;
- review against a real open-source integration when licensing permits.
