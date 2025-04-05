# NIPs

NIPs stand for **Nostr Implementation Possibilities**. They exist to document what MUST, what SHOULD and what MAY be implemented by [Nostr](https://github.com/fiatjaf/nostr)-compatible _relay_ and _client_ software.

- [NIP-01: Basic protocol flow description](https://github.com/fiatjaf/nostr/blob/master/nips/01.md)
- [NIP-02: Contact List and Petnames](https://github.com/fiatjaf/nostr/blob/master/nips/02.md)
- [NIP-03: OpenTimestamps Attestations for Events](https://github.com/fiatjaf/nostr/blob/master/nips/03.md)
- [NIP-04: Encrypted Direct Message](https://github.com/fiatjaf/nostr/blob/master/nips/04.md)
- [NIP-05: Mapping Nostr keys to DNS-based internet identifiers](https://github.com/fiatjaf/nostr/blob/master/nips/05.md)
- [NIP-06: Basic key derivation from mnemonic seed phrase](https://github.com/fiatjaf/nostr/blob/master/nips/06.md)
- [NIP-08: Handling Mentions](https://github.com/fiatjaf/nostr/blob/master/nips/08.md)
- [NIP-09: Event Deletion](https://github.com/fiatjaf/nostr/blob/master/nips/09.md)
- [NIP-11: Relay Information Document](https://github.com/fiatjaf/nostr/blob/master/nips/11.md)
- [NIP-12: Generic Tag Queries](https://github.com/fiatjaf/nostr/blob/master/nips/12.md)

## Event Kinds

| kind | description               | NIP |
|------|---------------------------|-----|
| 0    | Metadata                  | 5   |
| 1    | Text                      | 1   |
| 3    | Contacts                  | 2   |
| 4    | Encrypted Direct Messages | 4   |
| 5    | Event Deletion            | 9   |

Please update this list when proposing NIPs introducing new event kinds.
