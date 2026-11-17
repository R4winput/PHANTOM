# Privacy model

PHANTOM aims to minimize persistent identity and data. The foundation has no networking, message store, or telemetry. Future milestones must preserve the following design constraints:

- No account or permanent user name is required.
- Generate a new peer identity for each session; do not reuse an identity across conversations.
- The coordinator handles invitations and signaling, not message plaintext or private keys.
- Do not generate local chat history by default.
- Keep logs useful for diagnosis without including plaintext, keys, invitation tokens, or unnecessary peer identifiers.
- Review crash dumps, swap, temporary files, and packet captures as possible sources of persistence.

Dropping or zeroizing a buffer cannot by itself prove that all copies have disappeared from physical memory. Network observers may still see endpoint addresses, timing, sizes, or the existence of traffic. PHANTOM is not an anonymity system.
