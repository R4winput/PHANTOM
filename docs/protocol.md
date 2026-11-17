# Protocol notes

The first foundation release defines no network protocol. This page records the v0.1 design constraints so that wire behavior is explicit before it is implemented.

## Message envelope

Future messages should be versioned and typed, and include a session identifier, monotonically checked sequence/counter for replay handling, and an operation-specific payload. Session identifiers are presentation/routing values, not secrets or authentication credentials. Cryptographic fields depend on the operation and must be reviewed before serialization is fixed.

## Candidate message types

| Message | Direction | Purpose |
| --- | --- | --- |
| `JOIN_REQUEST` | joining peer → coordinator | Request admission using a temporary invitation |
| `JOIN_ACCEPTED` | coordinator → joining peer | Confirm admission |
| `PEER_ANNOUNCE` | coordinator → existing peer | Notify of a joining peer |
| `SESSION_READY` | peer ↔ peer | Confirm the authenticated channel is ready |
| `DATA_MESSAGE` | peer ↔ peer | Carry encrypted message content |
| `LEAVE_SESSION` | peer → coordinator/peer | Notify departure |
| `SESSION_CLOSED` | peer ↔ peer | Finish the session |

These names are illustrative, not a stable wire contract. v0.1 targets two participants, one-time expiring invitations, rejection of replayed messages, and no generated history. Avoid putting plaintext or secret material in coordinator messages or logs.
