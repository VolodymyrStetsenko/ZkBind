# Security Policy

## Support status

ZKBind is under active development. The current implementation performs Solidity verifier call-site discovery and does not yet classify cross-layer vulnerabilities automatically.

Automated output must be validated against the protocol specification, circuit semantics, verifier configuration, deployment state, and application flow before it is treated as a security finding.

Until the first tagged release, security fixes apply to the latest commit on `main`.

## Reporting a vulnerability in ZKBind

Report vulnerabilities privately when public disclosure could put ZKBind users or analyzed projects at risk.

Include, where possible:

- the affected commit or release;
- the affected component and file;
- technical impact;
- reproduction steps or a minimal test case;
- expected and actual behavior;
- suggested remediation;
- whether the issue affects ZKBind itself, its generated output, or a supported integration.

Do not include private keys, credentials, personal data, or live exploit details that could endanger third-party systems.

## Responsible use

ZKBind is intended for defensive research, authorized security reviews, development, and CI. Users are responsible for obtaining permission before testing contracts, infrastructure, applications, or deployments they do not own.

The project does not authorize:

- testing outside an explicit audit or bug-bounty scope;
- disrupting live services;
- accessing or moving third-party assets;
- publishing unverified allegations about a project;
- using generated test templates against live systems without authorization.

## Disclosure process

1. The report is acknowledged and triaged.
2. Impact and reproducibility are assessed.
3. A fix and regression test are prepared when the issue is confirmed.
4. Coordinated disclosure timing is agreed when third parties are affected.
5. A security advisory or release note is published when appropriate.

No reward program is currently offered. Attribution is provided on request when coordinated disclosure requirements are followed.
