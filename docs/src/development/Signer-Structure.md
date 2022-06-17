# Signer structure

## Architectural structure

On top level, Signer consists of following parts:

1. Rust backend core
2. FFI interface
3. Native frontend
4. Database

### Rust backend

There are 3 actual endpoints in `rust` folder: `signer`, which is source of
library used for Signer itself; `generate_message`, which is used to update
Signer repo with new built-in network information and to generate
over-the-airgap updates; and `qr_reader_pc` which is a minimalistic app to parse
qr codes that we had to write since there was no reasonably working alternative.

Sub-folders of the `rust` folder:

- `constants` — constant values defined for the whole workspace.
- `db_handling` — all database-related operations for Signer and
  generate_message tool. Most of the business logic is contained here.
- `defaults` — built-in and test data for database
- `definitions` — objects used across the workspace are defined here
- `files` — contains test files and is used for build and update generation
  processes. Most contents are gitignored.
- `generate_message` — tool to generate over-the-airgap updates and maintain
  network info database on hot side
- `navigator` — navigation for Signer app; it is realized in rust to unify app
  behavior across the platforms
- `parser` - parses signable transactions. This is internal logic for
  transaction_parsing that is used when signable transaction is identified, but
it could be used as a standalone lib for the same purpose.
- `printing_balance` — small lib to render tokens with proper units
- `qr_reader_pc` — small standalone PC app to parse QR codes in Signer
  ecosystem. Also is capable of parsing multiframe payloads (theoretically, in
practice it is not feasible due to PC webcam low performance)
- `qr_reader_phone` — logic to parse QR payloads in Signer
- `qrcode_rtx` — multiframe erasure-encoded payload generator for signer update
  QR animation.
- `qrcode_static` — generation of static qr codes used all over the qorkspace
- `signer` — FFI interface crate to generate bindings that bridge native code
  and rust backend
- `transaction_parsing` — high-level parser for all QR payloads sent into Signer
- `transaction_signing` — all operations that could be performed when user
  accepts payload parsed with transaction_parsing

### FFI interface

For interfacing rust code and native interface we use
[uniffi](https://mozilla.github.io/uniffi-rs/) framework. It is a framework
intended to aid building cross-platform software in Rust especially for the
cases of re-using components written in Rust in the smartphone application
development contexts. Other than Signer itself one of the most notable users of
the `uniffi` framework are the [Mozilla Application Services](
https://github.com/mozilla/application-services/)

`uniffi` framework provides a way for the developer to define a clear and a
typesafe `FFI` interface between components written in `Rust` and languates such
as `Kotlin` and `Swift`. This approach leads to a much more robust architecture
than implementing a homegrown FFI with, say, passing JSON-serialized data back
and forth between `Kotlin` and `Rust` code. Here is why.

Suppose the application needs to pass a following structure through FFI from
`Kotlin` to `Rust` or back:

```rust,noplaypen #[derive(Serialize, Deserialize)] struct Address { street:
String, city: String, } ```

This would mean that on the `Kotlin` side of the FFI there would have to be some
way of turning this type from JSON into a `Kotlin` type. It may be some sort of
scheme or even a manual JSON value-by-key data extraction.

Now suppose this struct is changed by adding and removing some fields:

```rust,noplaypen #[derive(Serialize, Deserialize)] struct Address { country:
String, city: String, index: usize, } ```

After this change on a Rust-side the developer would have to _remember_ to
reflect these changes on the `Kotlin` and `Swift` sides and if that is no done
there is a chance that it will not be caught in build-time by CI. It is quite
hard to remember everything and having a guarantee that such things would be
caught at compile time is much better than not having this sort of guarantee.
One of the things `uniffi` solves is exactly this: it provides compile-time
guarantees of typesafety.

The other concern with the JSON serialization approach is performance. As long
as small objects are transfered back and forth it is no trouble encoding them
into strings.  But suppose the application requires transfering bigger blobs of
binary data such as `png` images or even some metadata files. Using JSON would
force the developer to encode such blobs as `Strings` before passing them into
FFI and decoding them back into binary blobs on the other side of the FFI.
`uniffi` helps to avoid this also.



### Native frontend

Native frontends are made separately for each supported platform. To keep things
uniform, interfaces are made as simple as possible and as much code is written
in unified Rust component, as possible. Yet, platform-specific functions,
including runtime management and threading, are also accessed through native
framework. The structure of native frontend follows modern (2022) reactive
design pattern of View-Action-Model triad. Thus, all backend is located in data
model section, along with few native business logic components.

It is important to note, that native navigation is **not** used, as due to
subtle differences in its seemingly uniform design across platforms. Navigation
is instead implemented on Rust side and, as an additional advantage, is tested
there at lower computational cost for CI pipelines.

### Database

For storage of all data except secrets, a sled database is used. Choice of db
was based on its lightweightness, reliability, portability.

**TODO db structure here**

## Functional structure

Signer has following systems:

- Secure key management
- Signing
- Transaction parsing
- Transaction visualization
- Airgap data transfer
- Airgap updating
- Network detector
- Logging
- Self-signing updating capability
- UI

These systems are located in different parts the app and some of them rely on
hot-side infrastructure. The general design goal was to isolate as much as
possible in easily maintainable Rust code and keep only necessary functions in
native side. Currently those include:

- Hardware secret storage: we rely on hardware designer's KMS in accordance with
  best practices
- Network detector: network operations are limited by OS and we try to keep
  network access permissions for the app to minimum while still maintaining
simple breach detection
- Camera: currently image capture and recognition systems implementations in
  native environments by far surpass 3rd party ones. This might change in
future, but at least image capture will be managed by OS to maintain platform
compatibility.
- UI: we use native frameworks and components for rendering and user interaction
  for best look and feel of the app.

### Secure key management

Keypairs used in Signer are generated from secret seed phrase, derivation path
and optional secret password, in accordance with specifications described in
**subkey manual** using code imported directly from substrate codebase for best
conformance.

#### Secret seed phrase storage

Secret seed phrase is stored as a string in devices original KMS. It is
symmetrically encrypted with a strong key that either is stored in a
hardware-protected keyring or uses biometric data (in case of legacy android
devices without strongbox system). Secrets access is managed by operating
system's built-in authorization interface. Authorization is required for
creation of seeds, access to seeds and removal of seeds. One particular special
case is addition of the first seed on iOS platform, that does not trigger
authorization mechanism as the storage is empty at this moment; this is in
agreement with iOS key management system design and potentially leads to a
threat of attacker replacing a single key by adding it to empty device; this
attack is countered by authorization on seed removal.

Thus, secret seeds source of truth is KMS. To synchronize the rest of the app,
list of seed identifiers is sent to backend on app startup and on all events
related to changes in this list by calling **TODO corresponding function**.

Randopm seed generator and seed recovery tools are implemented in Rust. These
are the only 2 cases where seed originates not in KMS.

#### Derivation path management

The most complex part of key management is storage of derivation strings and
public keys. Improper handling here may lead to user's loss of control over
their assets.

Key records are stored as strings in database associated with secret seed
identifiers, crypto algorithm, and list of allowed networks. Public key and its
cryptographic algorithm are used to deterministically generate database record
key - thus by design distinct key entries directly correspond to addresses on
chain.

Creation of new records requires generation of public keys through derivation
process, thus secret seed should be queried - so adding items to this database
requires authentication.

Substrate keys could be natively used across all networks supporting their
crypto algorithm. This may lead to accidental re-use of keys; thus it is not
forbidden by the app, but networks are isolated unless user explicitly expresses
desire to enable key in given network. From user side, it is abstracted into
creation of independent addresses; however, real implementation stores addresses
with publik keys as storage keys and thus does not distinguish between networks.
To isolate networks, each key stores a field with a list of allowed networks,
and when user "creates" address with the same pubkey as already existing one, it
is just another network added to the list of networks.

**TODO here should be more discussion on key management features and/or links to
rustdocs**

**TODO bulk key import**

#### Optional password

Optional password (part of derivation path after `///`) is never stored, only
addresses that have password in their derivation path are marked. Thus, password
is queried every time it is needed with a tool separate from OS authentication
interface, but together with authentication screen, as password is always used
with a secret seed phrase.

#### Memory safety in secure key management

All memory handles by native framework relies on native framework's memory
protection mechanisms (JVM virtualization and Swift isolation and garbage
collection). However, when secrets are processen in Rust, no inherent designed
memory safety reatures are available. To prevent secrets remaining in memory
after their use, `zeroize` library is used. Also, **describe string destruction
protocol or fix it**

### Signing

Every payload to be signed is first extracted from transfer payload in agreement
with uos specification and polkadot-js implementation. Only payloads that could
be parsed and visualized somehow could be signed to avoid blind signing - thus
on parser error no signable payload is produced and signing procedure is not
initiated.

When signable payload is ready, it is stored in **TODO database tree** while
user makes decision on whether to sign it. While in storage, database checksum
is monitored for changes.

Signing uses private key generated from KMS-protected secret seed phrase,
derivation string and optional password. Signing operation itself is imported
directly from substrate codebase as dependency.

Signing event or its failure is logged and signature wrapped in uos format is
presented as a qr static image on the phone.

### Transaction parsing

**TODO: dump text from crate**

### Transaction visualization

Signable transaction is decomposed into hierarchical cards for clarity. All
possible scale-decodable types are assigned to generalized cisualization
patterns ("transaction cards") with some types having special visualizations
(`balance` formatted with proper decimals and units, identicons added to
identities, etc). Each card is assigned `order` and `indent` that allow the
cards to be shown in a lazy view environment. Thus, any networks that have
minimal metadata requirements should be decodable and visualizeable.

Some cards also include documentation entries fetched from metadata. Those could
be expanded in UI on touch.

Thus, user has opportunity to read the whole transaction before signing.

### Airgap data transfer

Transactions are encoded in accordance to uos standard in QR codes. QR codes can
be sent into Signer - through static frames or dynamic multiframe animations -
and back - only as static frames. QR codes are decoded through native image
recognition system and decoded through rust backend; output QR codes are
generated in png format by backend. There are 2 formats of multiframe QR codes:
legacy multiframe and `raptorq` multiframe. Legacy multiframe format requires
all frames in animation to be collected and is thus inpractical for larger
payloads. Raptorq multiframe format allows any subset of frames to be collected
and thus allows large payloads to be transferred effortlessly.

Fast multiframe transfer works efficiently at 30fps. Typical large payloads
contain up to 200 frames at current state of networks. This can be theoretically
performed in under 10 seconds; practically this works in under 1 minute.

### Airgap updating

Signer can download new networks and metadata updates from QR data. To prevent
malicious updates from compromising security, a system of certificates is
implemented. 

**TODO stuff from certificate docs**

### Network detector

An additional security feature is network detector. When the app is on, it runs
in the background (on low-priority thread) and attempts to monitor the network
availability. This detector is implemented differently on different planforms
and has different features and limitations; however, it does not and could not
provide full connectivity monitoring and proper maintaining of airgap is
dependent on user. Signer device should always be kept in airplane mode and all
other connectivity should be disabled.

The basic idea of network detection alertness is that when network connectivity
is detected, 3 things happen:

1. Event is logged in history
2. Visual indication of network status is presented to user (shield in corner of
screen and message in alert activated by the shield)
3. Certain Signer functions are disabled (user authentication, seed and key
creation, etc.) - features that bring secret material into active app memory
from storage

When network connectivity is lost, only visual indication changes. To restore
clean state of Signer, user should acknowledge safety alert by pressing on
shield icon, reading and accepting the warning. Upon acknowledging, it is logged
in history, visual indication changes to green and all normal Signer functions
are restored.

#### Network detector in iOS

Airplane mode detection in iOS is forbidden and may lead to expulsion of the app
from appstore. Thus, detector relies on probing network interfaces. If any
network interface is up, network alert is triggered.

#### Network detector in Android

Network detector is triggered directly by airplane mode change event.

#### Bluetooth, NFC, etc,

Other possible network connectivity methods are not monitored. Even though it is
possible to add detectors for them, accessing their status will require the app
to request corresponding permissions form OS, thus reducing app's isolation and
decreasing overal security - first, by increasing chance of leak in breach
event, and second, by making corrupt fake app that leaks information through
network appear more normal. Furthermore, there is information that network might
be connected through cable in some devices in airplane mode; there was no
research on what debugging through cable is capable of for devices in airplane
mode. Thus, network detector is a convenience too and should not be relied on as
sole source of security; user is responsible for device isolation.

### Logging

All events that happen in Signer are logged by backend in history tree of
database. From user interface, all events are presented in chronological order
on log screen. On the same screen, history checksum could be seen and custom
text entries could be added to database. Checksum uses time added to history
records in computation and is therefore impractical to forge.

Events presented on log screen are colored to distinguish "normal" and
"dangerous" events. Shown records give minimal important information about the
event. On click, detailed info screen is shown, where all events happened at the
same time are presented in detail (including transactions, that are decoded for
review if metadata is still available).

Log could also be erased for privacy; erasure event is logged and becomes the
first event in recorded history.

### Self-signing updating capability

Signer can sign network and metadata updates that could be used for other
signers. User can select any update component present in Signer and any key
available for any network and generate a qr code which, upon decoding, can be
used by `generate_message` or similar tool to generate over-the-airgap update.
**TODO what is in signature**

This feature was designed for elegance, but it is quite useful to maintain
update signing key for large update distribution centers, for it allows to
securely store secret sertificate key that could not be practically revoked if
compromised.

### UI

User interface is organized through View-Action-DataModel abstraction.

### View

Signer visual representation is abstracted in 3 visual layers placed on top of
each other: `screen`, `modal` and `alert`. This structure is mostly an
adaptation of iOS design guidelines, as android native UI is much flexible and
it is easier to adopt it to iOS design patterns than vice versa. Up to one of
each component could be presented simultaneously. Screen component is always
present in the app, but sometimes it is fully or partially blocked by other
components.

Modals and alerts are dismissed on `goBack` action, screens have complex
navigation rules. Modals require user to take action and interrupt flow. Alerts
are used for short information interruptions, like error messages or
confirmations.

In addition to these, header bar is always present on screen and footer bar is
presented in some cases. Footer bar always has same structure and only allows
navigation to one of navigation roots. Top bar might contain back button, screen
name, and extra menu button; status indicator is always shown on top bar.

### Action

Almost all actions available to user are in fact handled by single operation -
`action()` backend function, that is called through `pushButton` native
interface. In native side, this operation is debounced by time. On rust side,
actions are performed on static mutex storing app state; on blocked mutex
actions is ignored, as well as impossible actions that are not allowed in
current state of navigation. Thus, state of the app is protected against
undefined concurrency effects by hardware button-like behavior of `action()`.

Most actions lead to change of shown combination of screen, modal and alert; but
some actions - for example, those involving keyboard input - alter contents of a
UI component. In most cases, all parameters of UI components are passed as
states (more or less similar concept on all platforms) and frontend framework
detects updates and seamlessly performs proper rendering.

Action accepts 3 parameters: action type (enum), action data (&str), secret data
(&str). Secret data is used to transfer secret information and care is taken to
always properly zeroize its contents; on contrary, action data could contain
large strings and is optimized normally.
