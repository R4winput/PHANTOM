# Threat model v1

This document defines the design target before the first network feature. It is an initial model, not a claim that the current code already provides these properties.

## Assets

- Message content and attachments added in later milestones.
- Per-session private keys, ephemeral peer identities, and invitation tokens.
- Session membership and short-lived coordination data.
- Users' endpoint and network metadata.

## Adversaries

- A curious or compromised coordinator or relay.
- A passive network observer able to capture traffic.
- An active network attacker who can replay, alter, delay, or drop packets.
- An unauthorized person who obtains or reuses an invitation.
- A malicious participant who records content they receive.
- A local process, crash dump, or operating system that can observe endpoint memory or storage.

## Security objectives

| Area | Objective |
| --- | --- |
| Content confidentiality | A coordinator or relay cannot read message plaintext. |
| Session identity | Do not require accounts or permanent user names; create fresh identities per session. |
| Data minimization | Avoid unnecessary persistence of messages, keys, and session data. |
| Membership changes | A removed member must not decrypt messages sent after removal. This is a future group-session objective. |
| Invitation integrity | Reject invalid, expired, and reused invitations. |
| Message integrity | Reject altered and replayed messages. |
| Teardown | End the session and release resources when a participant exits. |

## Out of scope / limitations

- Hiding whether two endpoints communicate or concealing all traffic metadata.
- Preventing a recipient from copying, recording, or photographing received content.
- Protecting an endpoint already controlled by an attacker.
- Guaranteeing physical erasure when Rust values are dropped. Copies, swap, crash dumps, and OS behavior can preserve bytes.
- Preventing denial of service or guaranteeing network availability.

## Design direction

Use authenticated transport and fresh per-session peer identities. Keep the coordinator on the invitation/signaling path only. Specify message sequence numbers and replay behavior in the protocol before implementation. Do not log plaintext, secrets, invitation values, or unnecessary peer identifiers. Prefer audited protocol and cryptographic implementations; use OpenMLS for the later group milestone rather than inventing group cryptography.

## Validation plan

When the networking milestone begins, add reproducible tests for invalid, expired, and reused invitations; packet modification and replay; peer disconnect and teardown; reconnect attempts with prior identities; absence of chat history; and what a packet capture reveals to an observer. Document observed metadata and limitations rather than claiming anonymity.
