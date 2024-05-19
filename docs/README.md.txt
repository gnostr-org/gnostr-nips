NIPs

NIPs stand for Nostr Implementation Possibilities.

They exist to document what may be implemented by Nostr-compatible relay
and client software.

------------------------------------------------------------------------

-   List
-   Event Kinds
-   Message Types
    -   Client to Relay
    -   Relay to Client
-   Standardized Tags
-   Criteria for acceptance of NIPs
-   Is this repository a centralizing factor?
-   How this repository works
-   Breaking Changes
-   License

------------------------------------------------------------------------

List

-   NIP-01: Basic protocol flow description
-   NIP-02: Follow List
-   NIP-03: OpenTimestamps Attestations for Events
-   NIP-04: Encrypted Direct Message .txtash; unrecommended: deprecated
    in favor of NIP-17
-   NIP-05: Mapping Nostr keys to DNS-based internet identifiers
-   NIP-06: Basic key derivation from mnemonic seed phrase
-   NIP-07: window.nostr capability for web browsers
-   NIP-08: Handling Mentions .txtash; unrecommended: deprecated in favor
    of NIP-27
-   NIP-09: Event Deletion
-   NIP-10: Conventions for clients&rsquo; use of e and p tags in text
    events
-   NIP-11: Relay Information Document
-   NIP-13: Proof of Work
-   NIP-14: Subject tag in text events
-   NIP-15: Nostr Marketplace (for resilient marketplaces)
-   NIP-17: Private Direct Messages
-   NIP-18: Reposts
-   NIP-19: bech32-encoded entities
-   NIP-21: nostr: URI scheme
-   NIP-23: Long-form Content
-   NIP-24: Extra metadata fields and tags
-   NIP-25: Reactions
-   NIP-26: Delegated Event Signing
-   NIP-27: Text Note References
-   NIP-28: Public Chat
-   NIP-29: Relay-based Groups
-   NIP-30: Custom Emoji
-   NIP-31: Dealing with Unknown Events
-   NIP-32: Labeling
-   NIP-34: git stuff
-   NIP-36: Sensitive Content
-   NIP-38: User Statuses
-   NIP-39: External Identities in Profiles
-   NIP-40: Expiration Timestamp
-   NIP-42: Authentication of clients to relays
-   NIP-44: Versioned Encryption
-   NIP-45: Counting results
-   NIP-46: Nostr Connect
-   NIP-47: Wallet Connect
-   NIP-48: Proxy Tags
-   NIP-49: Private Key Encryption
-   NIP-50: Search Capability
-   NIP-51: Lists
-   NIP-52: Calendar Events
-   NIP-53: Live Activities
-   NIP-54: Wiki
-   NIP-56: Reporting
-   NIP-57: Lightning Zaps
-   NIP-58: Badges
-   NIP-59: Gift Wrap
-   NIP-65: Relay List Metadata
-   NIP-72: Moderated Communities
-   NIP-75: Zap Goals
-   NIP-78: Application-specific data
-   NIP-84: Highlights
-   NIP-89: Recommended Application Handlers
-   NIP-90: Data Vending Machines
-   NIP-92: Media Attachments
-   NIP-94: File Metadata
-   NIP-96: HTTP File Storage Integration
-   NIP-98: HTTP Auth
-   NIP-99: Classified Listings

Event Kinds

  ------------------------------------------------------------------------
  kind           description                   NIP
  -------------- ----------------------------- ---------------------------
  0              Metadata                      01

  1              Short Text Note               01

  2              Recommend Relay               01 (deprecated)

  3              Follows                       02

  4              Encrypted Direct Messages     04

  5              Event Deletion                09

  6              Repost                        18

  7              Reaction                      25

  8              Badge Award                   58

  9              Group Chat Message            29

  10             Group Chat Threaded Reply     29

  11             Group Thread                  29

  12             Group Thread Reply            29

  13             Seal                          59

  14             Direct Message                17

  16             Generic Repost                18

  40             Channel Creation              28

  41             Channel Metadata              28

  42             Channel Message               28

  43             Channel Hide Message          28

  44             Channel Mute User             28

  818            Merge Requests                54

  1021           Bid                           15

  1022           Bid confirmation              15

  1040           OpenTimestamps                03

  1059           Gift Wrap                     59

  1063           File Metadata                 94

  1311           Live Chat Message             53

  1617           Patches                       34

  1621           Issues                        34

  1622           Replies                       34

  1630-1633      Status                        34

  1971           Problem Tracker               nostrocket

  1984           Reporting                     56

  1985           Label                         32

  4550           Community Post Approval       72

  5000-5999      Job Request                   90

  6000-6999      Job Result                    90

  7000           Job Feedback                  90

  9000-9030      Group Control Events          29

  9041           Zap Goal                      75

  9734           Zap Request                   57

  9735           Zap                           57

  9802           Highlights                    84

  10000          Mute list                     51

  10001          Pin list                      51

  10002          Relay List Metadata           65

  10003          Bookmark list                 51

  10004          Communities list              51

  10005          Public chats list             51

  10006          Blocked relays list           51

  10007          Search relays list            51

  10009          User groups                   51, 29

  10015          Interests list                51

  10030          User emoji list               51

  10050          Relay list to receive DMs     17

  10096          File storage server list      96

  13194          Wallet Info                   47

  21000          Lightning Pub RPC             Lightning.Pub

  22242          Client Authentication         42

  23194          Wallet Request                47

  23195          Wallet Response               47

  24133          Nostr Connect                 46

  27235          HTTP Auth                     98

  30000          Follow sets                   51

  30001          Generic lists                 51

  30002          Relay sets                    51

  30003          Bookmark sets                 51

  30004          Curation sets                 51

  30008          Profile Badges                58

  30009          Badge Definition              58

  30015          Interest sets                 51

  30017          Create or update a stall      15

  30018          Create or update a product    15

  30019          Marketplace UI/UX             15

  30020          Product sold as an auction    15

  30023          Long-form Content             23

  30024          Draft Long-form Content       23

  30030          Emoji sets                    51

  30063          Release artifact sets         51

  30078          Application-specific Data     78

  30311          Live Event                    53

  30315          User Statuses                 38

  30402          Classified Listing            99

  30403          Draft Classified Listing      99

  30617          Repository announcements      34

  30818          Wiki article                  54

  30819          Redirects                     54

  31922          Date-Based Calendar Event     52

  31923          Time-Based Calendar Event     52

  31924          Calendar                      52

  31925          Calendar Event RSVP           52

  31989          Handler recommendation        89

  31990          Handler information           89

  34550          Community Definition          72

  39000-9        Group metadata events         29
  ------------------------------------------------------------------------

Message types

Client to Relay

  ------------------------------------------------------------------------
  type    description                                          NIP
  ------- ---------------------------------------------------- -----------
  EVENT   used to publish events                               01

  REQ     used to request events and subscribe to new updates  01

  CLOSE   used to stop previous subscriptions                  01

  AUTH    used to send authentication events                   42

  COUNT   used to request event counts                         45
  ------------------------------------------------------------------------

Relay to Client

  ------------------------------------------------------------------------
  type     description                                          NIP
  -------- ---------------------------------------------------- ----------
  EOSE     used to notify clients all stored events have been   01
           sent                                                 

  EVENT    used to send events requested to clients             01

  NOTICE   used to send human-readable messages to clients      01

  OK       used to notify clients if an EVENT was successful    01

  CLOSED   used to notify clients that a REQ was ended and why  01

  AUTH     used to send authentication challenges               42

  COUNT    used to send requested event counts to clients       45
  ------------------------------------------------------------------------

Please update these lists when proposing NIPs introducing new event
kinds.

Standardized Tags

  -------------------------------------------------------------------------------
  name              value                   other         NIP
                                            parameters    
  ----------------- ----------------------- ------------- -----------------------
  e                 event id (hex)          relay URL,    01, 10
                                            marker        

  p                 pubkey (hex)            relay URL,    01, 02
                                            petname       

  a                 coordinates to an event relay URL     01

  d                 identifier              &ndash;       01

  g                 geohash                 &ndash;       52

  i                 identity                proof         39

  k                 kind number (string)    &ndash;       18, 25, 72

  l                 label, label namespace  annotations   32

  L                 label namespace         &ndash;       32

  m                 MIME type               &ndash;       94

  q                 event id (hex)          relay URL     18

  r                 a reference (URL, etc)  petname       

  r                 relay url               marker        65

  t                 hashtag                 &ndash;       

  alt               summary                 &ndash;       31

  amount            millisatoshis,          &ndash;       57
                    stringified                           

  bolt11            bolt11 invoice          &ndash;       57

  challenge         challenge string        &ndash;       42

  client            name, address           relay URL     89

  clone             git clone URL           &ndash;       34

  content-warning   reason                  &ndash;       36

  delegation        pubkey, conditions,     &ndash;       26
                    delegation token                      

  description       description             &ndash;       34, 57, 58

  emoji             shortcode, image URL    &ndash;       30

  encrypted         &ndash;                 &ndash;       90

  expiration        unix timestamp (string) &ndash;       40

  goal              event id (hex)          relay URL     75

  image             image URL               dimensions in 23, 58
                                            pixels        

  imeta             inline metadata         &ndash;       92

  lnurl             bech32 encoded lnurl    &ndash;       57

  location          location string         &ndash;       52, 99

  name              name                    &ndash;       34, 58

  nonce             random                  &ndash;       13

  preimage          hash of bolt11 invoice  &ndash;       57

  price             price                   currency,     99
                                            frequency     

  proxy             external ID             protocol      48

  published_at      unix timestamp (string) &ndash;       23

  relay             relay url               &ndash;       42, 17

  relays            relay list              &ndash;       57

  server            file storage server url &ndash;       96

  subject           subject                 &ndash;       14, 17

  summary           article summary         &ndash;       23

  thumb             badge thumbnail         dimensions in 58
                                            pixels        

  title             article title           &ndash;       23

  web               webpage URL             &ndash;       34

  zap               pubkey (hex), relay URL weight        57
  -------------------------------------------------------------------------------

Criteria for acceptance of NIPs

1.  They should be implemented in at least two clients and one relay
    &ndash; when applicable.
2.  They should make sense.
3.  They should be optional and backwards-compatible: care must be taken
    such that clients and relays that choose to not implement them do
    not stop working when interacting with the ones that choose to.
4.  There should be no more than one way of doing the same thing.
5.  Other rules will be made up when necessary.

Is this repository a centralizing factor?

To promote interoperability, we standards that everybody can follow, and
we need them to define a single way of doing each thing without ever
hurting backwards-compatibility, and for that purpose there is no way
around getting everybody to agree on the same thing and keep a
centralized index of these standards. However the fact that such index
exists doesn&rsquo;t hurt the decentralization of Nostr. At any point
the central index can be challenged if it is failing to fulfill the
needs of the protocol and it can migrate to other places and be
maintained by other people.

It can even fork into multiple and then some clients would go one way,
others would go another way, and some clients would adhere to both
competing standards. This would hurt the simplicity, openness and
interoperability of Nostr a little, but everything would still work in
the short term.

There is a list of notable Nostr software developers who have commit
access to this repository, but that exists mostly for practical reasons,
as by the nature of the thing we&rsquo;re dealing with the repository
owner can revoke membership and rewrite history as they want &ndash; and
if these actions are unjustified or perceived as bad or evil the
community must react.

How this repository works

Standards may emerge in two ways: the first way is that someone starts
doing something, then others copy it; the second way is that someone has
an idea of a new standard that could benefit multiple clients and the
protocol in general without breaking backwards-compatibility and the
principle of having a single way of doing things, then they write that
idea and submit it to this repository, other interested parties read it
and give their feedback, then once most people reasonably agree we
codify that in a NIP which client and relay developers that are
interested in the feature can proceed to implement.

These two ways of standardizing things are supported by this repository.
Although the second is preferred, an effort will be made to codify
standards emerged outside this repository into NIPs that can be later
referenced and easily understood and implemented by others &ndash; but
obviously as in any human system discretion may be applied when
standards are considered harmful.

Breaking Changes

Breaking Changes

License

All NIPs are public domain.

Contributors
