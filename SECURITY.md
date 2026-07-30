# Security Policy

## Project status

ZKBind is an early research prototype. Scanner output may be incomplete, experimental, or require manual confirmation. Do not treat an automated result as a confirmed vulnerability without validating the project specification, circuit semantics, verifier configuration, deployment state, and application flow.

## Reporting a vulnerability in ZKBind

Please report vulnerabilities privately before opening a public issue when disclosure could put users or integrated projects at risk.

Include, where possible:

- affected commit or release;
- affected component and file;
- technical impact;
- reproduction steps or a minimal test case;
- expected and actual behavior;
- suggested remediation;
- whether the issue affects ZKBind itself, generated output, or a supported integration.

Do not include private keys, credentials, personal data, or live exploit details that could endanger third-party systems.

## Responsible research

ZKBind is intended for defensive research, authorized security reviews, development, and CI. Users are responsible for obtaining permission before testing contracts, infrastructure, applications, or deployments they do not own.

The project does not authorize:

- testing outside an explicit audit or bug-bounty scope;
- disrupting live services;
- accessing or moving third-party assets;
- publishing unverified allegations about a project;
- using generated test templates as turnkey exploits against live systems.

## Supported versions

Until the first tagged release, security fixes apply to the latest commit on `main`. A formal supported-version table will be introduced with versioned releases.

## Disclosure process

1. The report is acknowledged and triaged.
2. Impact and reproducibility are assessed.
3. A fix and regression test are prepared when confirmed.
4. Coordinated disclosure timing is agreed when third parties are affected.
5. A security advisory or release note is published when appropriate.

No reward program is currently offered. Attribution will be provided when requested and when coordinated disclosure requirements are followed.
