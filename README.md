# PHANTOM

**Ephemeral P2P messaging, designed with a security research mindset.**

PHANTOM is an experimental Rust project for authenticated, encrypted, short-lived conversations. It is being built in documented stages: first a clean workspace and threat model, then a two-peer prototype, and only later group messaging and encrypted file transfer.

> **Status:** foundation (`0.1.0`) in progress. The current version is not a messaging application and makes no production security claims.

## Project goals

- No accounts or permanent user names.
- Fresh peer identities for each conversation.
- Keep messages, keys, and session data in memory only where possible.
- Keep the coordinator out of the plaintext path; it handles invitations and signaling.
- State security guarantees and limitations plainly.

PHANTOM cannot prevent a participant from copying or photographing received content. Network observers may still learn that endpoints communicate. Releasing Rust memory does not guarantee physical erasure: swap, crash dumps, compiler copies, and operating-system behavior matter. See the [threat model](docs/threat-model.md) before relying on any guarantee.

## Workspace

| Crate | Responsibility |
| --- | --- |
| `phantom-cli` | User-facing command line |
| `phantom-core` | Session lifecycle and orchestration |
| `phantom-network` | Peer connections and transport |
| `phantom-crypto` | Session identities and cryptographic operations |
| `phantom-protocol` | Versioned, typed wire messages |
| `phantom-coordinator` | Standalone invitation and signaling service |

The coordinator is a separate binary and may be hosted on a public machine. It must not receive message plaintext or session keys.

## Getting started

Install Rust 1.85 or later, then run:

```sh
cargo run -p phantom-cli -- --version
```

The initial CLI is intentionally small. Networking is not implemented in this milestone.

## Development

```sh
cargo fmt --all -- --check
cargo test --workspace
```

Pull requests run the same formatting and workspace test commands in GitHub Actions. See [architecture](docs/architecture.md), [protocol notes](docs/protocol.md), [privacy model](docs/privacy-model.md), and [security policy](SECURITY.md).

## Roadmap

1. **Rust foundation:** workspace, six crates, CLI version command, threat model, CI.
2. **Networking:** two peers, QUIC first, NAT traversal and relay fallback.
3. **Session security:** one-time invitations, ephemeral identities, authenticated encrypted sessions, replay protection, reliable teardown.
4. **Privacy research:** metadata and log review, secret lifetime review, persistence tests and documented packet captures.
5. **Later milestones:** OpenMLS group messaging, then encrypted file transfer.

PHANTOM v0.1 targets two independent machines. It excludes group conversations, file transfer, history, and a TUI. QUIC is preferred, but UDP-blocked networks require a separately evaluated relay fallback. We will not invent a cryptographic protocol; group messaging is planned around OpenMLS after the two-peer channel is established.

## License

PHANTOM is distributed under the MIT License. See [LICENSE](LICENSE).
