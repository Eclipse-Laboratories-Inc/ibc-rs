# CHANGELOG

## v0.40.0

*May 8, 2023*

This release primarily consolidated the modules in the ibc-rs crate, removed many legacy items, and documented every item in the crate. This represents a big step towards v1.0. Very few items changed name; most were just moved to elsewhere in the module tree. Perhaps a good heuristic to fix the breaking changes is the remove the faulty `use` statements, and have your editor re-import the item.

There were also a few minor validation checks missing, which we added. These were pretty much the last remaining known ones.

There are breaking changes.

### BREAKING CHANGES

- Add missing validation checks for all the IBC message types
  ([#233](https://github.com/cosmos/ibc-rs/issues/233))
- Reduce and consolidate the amount of public modules exposed
  ([#235](https://github.com/cosmos/ibc-rs/issues/235))
- Separate validation/execution handlers from context API
  ([#539](https://github.com/cosmos/ibc-rs/issues/539))
- Make `TYPE_URL`s private ([#597](https://github.com/cosmos/ibc-rs/issues/597))

### FEATURES

- Add parity-scale-codec, borsh, serde feature for *Path
  ([#652](https://github.com/cosmos/ibc-rs/issues/652))

### IMPROVEMENTS

- Document every method of `ValidationContext` and `ExecutionContext`
  ([#376](https://github.com/cosmos/ibc-rs/issues/376))

## v0.39.0

*May 2, 2023*

This release primarily adds support for the `memo` field to the token transfer
app (ICS 20). This required updating ibc-proto-rs and tendermint-rs dependencies
as well.

There are consensus-breaking changes.

### BREAKING CHANGES

- Bump ibc-proto to v0.29.0, bump tendermint to v0.30.0, and add `memo` field to
  `PacketData` ([#559](https://github.com/cosmos/ibc-rs/issues/559))
- Add missing `ClientType` and `ClientId` validation checks
  ([#621](https://github.com/cosmos/ibc-rs/issues/621))

### FEATURES

- Define a new `ValidationContext::validate_message_signer` method to allow
  validation of the `signer` field in messages across all handlers.
  ([#619](https://github.com/cosmos/ibc-rs/issues/619))

## v0.38.0

*April 24, 2023*

This release involves splitting the newly defined `MsgUpdateClient` type in
v0.37.0 into distinct IBC message structs: `MsgUpdateClient` and
`MsgSubmitMisbehaviour`. Additionally, we made improvements to the `Version`
validations in connection and channel handshakes, discarded now-unused
`store_client_type` interface, and removed `IbcEventType` to enable each IBC
event variant to define its own set of event types.

There are consensus-breaking changes

### BREAKING CHANGES

- Remove `store_client_type` interface as it is not included in the IBC spec anymore.
  ([#592](https://github.com/cosmos/ibc-rs/issues/592))
- Code clean-up remained from v0.37.0 release
- ([#622](https://github.com/cosmos/ibc-rs/issues/622))
- Remove `IbcEventType` ([#623](https://github.com/cosmos/ibc-rs/issues/623))
- Split `MsgUpdateClient` back into `MsgUpdateClient` and `MsgSubmitMisbehaviour`
  ([#628](https://github.com/cosmos/ibc-rs/issues/628))
- Refactor and fix version validation in connection and channel handshakes
  ([#625](https://github.com/cosmos/ibc-rs/issues/625))

### IMPROVEMENTS

- Make token transfer events compatible with latest ibc-go
  ([#495](https://github.com/cosmos/ibc-rs/pull/495))

## v0.37.0

*April 13, 2023*

This release primarily updates `ClientState` to adopt a better API for client updates and misbehaviour detection, borrowed from ibc-go's ADR 6. In the process of updating the API, a few bugs were found in the tendermint light client and fixed.

There are consensus-breaking changes.

### BREAKING CHANGES

- `ClientState`: Split `check_misbehaviour_and_update_state` 
  and `check_header_and_update_state`
  ([#535](https://github.com/cosmos/ibc-rs/issues/535))
- Improve MsgTransfer struct
  ([#567](https://github.com/cosmos/ibc-rs/issues/567))
- Remove `ics05_port::context::PortReader` ([#580](https://github.com/cosmos/ibc-rs/issues/580))
- Check if `ClientStatePath` is empty during client creation process
  ([#604](https://github.com/cosmos/ibc-rs/issues/604))

### BUG FIXES

- Disallow creation of new Tendermint client state instance with a frozen height
 ([#178](https://github.com/cosmos/ibc-rs/issues/178))
- Emit a message event for SendPacket ([#574](https://github.com/cosmos/ibc-rs/issues/574))
- Properly convert from `Any` to `MsgEnvelope` 
  ([#578](https://github.com/cosmos/ibc-rs/issues/578))
- Tendermint light client: fix missing trusted_validator_set 
  hash check
  ([#583](https://github.com/cosmos/ibc-rs/issues/583))
- Tendermint light client: fix missing `Header.height()` 
  vs `Header.trusted_height` check
  ([#585](https://github.com/cosmos/ibc-rs/issues/585))
- Tendermint light client: ensure that we use the correct
  chain ID in commit verification
  ([#589](https://github.com/cosmos/ibc-rs/issues/589))
- tx_msg: Remove panic in `Msg::get_sign_bytes`
  ([#593](https://github.com/cosmos/ibc-rs/issues/593))
- Tendermint light client: add check that ensure that
  the consensus state timestamps are monotonic, otherwise
  freeze the client
  ([#598](https://github.com/cosmos/ibc-rs/issues/598))
- Tendermint light client: fix how the client's latest
  height is updated
  ([#601](https://github.com/cosmos/ibc-rs/issues/601))

### FEATURES

- Prefixed denom parity scale codec enabled
  ([#577](https://github.com/cosmos/ibc-rs/pull/577))
- Add (de)serialization for `ics04_channel::handler::ModuleExtras`
  ([#581](https://github.com/cosmos/ibc-rs/issues/581))

## v0.36.0

*March 27, 2023*

This release adds the emission a `"message"` event for all handlers, which hermes currently
depends on.

There are no consensus-breaking changes.

### BUG

- Emit a message event for each IBC handling
  ([#563](https://github.com/cosmos/ibc-rs/issues/563))

## v0.35.0

*March 22, 2023*

This release fixes a bug in the packet timeout handler.

This is a consensus-breaking change.

### BUG

- Timeout handler returns an error only when both height and timestamp have not reached yet 
  ([#555](https://github.com/cosmos/ibc-rs/issues/555))

## v0.34.0

*March 17, 2023*

This release fixes a bug in the connection handshake.

This is a consensus-breaking change.

### BUG

- Fix client IDs for the proof verifications in `ConnectionOpenTry` and `ConnectionOpenAck` 
([#550](https://github.com/cosmos/ibc-rs/issues/550))

## v0.33.0

*March 16, 2023*

This release primarily updates the `ClientState` trait.

There are no consensus-breaking changes.

### BREAKING CHANGES

- Replace specific verify_functions inside `ics02_client` with generic
  `verify_membership` and `verify_non_membership` interfaces.
  ([#530](https://github.com/cosmos/ibc-rs/issues/530))
- Replace `ClientState::frozen_height()` and `ClientState::is_frozen()`
  with `ClientState::confirm_frozen()`
  ([#545](https://github.com/cosmos/ibc-rs/issues/545))

### IMPROVEMENT

- Fix `ContextError` Display output 
  ([#547](https://github.com/cosmos/ibc-rs/issues/547))

## v0.32.0

*March 9, 2023*

This release primarily removes the `'static` lifetime bound on the `Module` trait,
and adds some methods to the token transfer validation trait.

There are no consensus-breaking changes.

### BREAKING CHANGES

- Move `verify_delay_passed` process and its associated errors under the
  `ics03_connection` section and reduce entanglements with the
  `ValidationContext`.
  ([#404](https://github.com/cosmos/ibc-rs/issues/404))
- Refactor and privatize Packet/Ack commitment computations for improved security
  and modularity.
  ([#470](https://github.com/cosmos/ibc-rs/issues/470))
- Allow for non-'static bound Modules 
  [#490](https://github.com/cosmos/ibc-rs/issues/490))
- Separate the validation from the execution process for `send/mint/burn_coins`
  operations.
  ([#502](https://github.com/cosmos/ibc-rs/issues/502))
- Refactor naming in the Transfer application to align with the repo naming
  conventions.
  ([#506](https://github.com/cosmos/ibc-rs/issues/506))
- Refactor `is_send/receive_enabled` interfaces within the transfer application
  to `can_send/receive_coins` returning `Result<(), TokenTransferError>` type
  for a better failure handler
  ([#508](https://github.com/cosmos/ibc-rs/issues/508))

### IMPROVEMENTS

- Use `<&str>::deserialize` instead of `String::deserialize` to avoid an extra
  allocation ([#496](https://github.com/cosmos/ibc-rs/issues/496))
- In `test_serialization_roundtrip`, check that round-tripped data is equal
  ([#497](https://github.com/cosmos/ibc-rs/issues/497))

## v0.31.0

*February 28, 2023*

This release contains quality of life improvements.

There are no consensus-breaking changes.

### BREAKING CHANGES

- Remove ibc::handler module ([#478](https://github.com/cosmos/ibc-rs/issues/478))
- Discard the `connection-channels` method under `ValidationContext` since it is
  no longer used by the core handlers. 
  ([#479](https://github.com/cosmos/ibc-rs/issues/479))
- Remove Send + Sync supertraits on the Module trait
  ([#480](https://github.com/cosmos/ibc-rs/issues/480))
- Modify `validate_self_client` error type to return `ContextError` instead of
  `ConnectionError` 
  ([#482](https://github.com/cosmos/ibc-rs/issues/482))

### IMPROVEMENTS

- Fix typos ([\#472](https://github.com/cosmos/ibc-rs/issues/472))

## v0.30.0

*February 24, 2023*

This release contains an overhaul of the `send_packet()` and `send_transfer()` architecture.
The main gain is to separate into `send_packet_{validate,execute}()`, and similarly for 
`send_transfer()`.

There are no consensus-breaking changes.

### BREAKING CHANGES

- Update send_packet/transfer(), and related contexts
  ([#442](https://github.com/cosmos/ibc-rs/issues/442))

## v0.29.0

*February 22, 2023*

This release includes the latest Tendermint-rs v0.29.0 and removes the
`Reader` and `Keeper` API in favor of the new `ValidationContext`/`ExecutionContext` API as the default.
Additionally, unit tests have been updated to work with the new API.

There are consensus-breaking changes.

### BREAKING CHANGES

- Remove Reader and Keeper API
  ([#279](https://github.com/cosmos/ibc-rs/issues/279))
- Refactor `get_*` and `store_*` methods to take `*Path` structs instead
  ([#382](https://github.com/cosmos/ibc-rs/issues/382))
- Make `ValidationContext::host_timestamp()` abstract and remove
  `ValidationContext::pending_host_consensus_state()`
  ([#418](https://github.com/cosmos/ibc-rs/issues/418))

### BUG FIXES

- Mend error variant todo!()s wherever tendermint client calls the
  "consensus_state" method
  ([#403](https://github.com/cosmos/ibc-rs/issues/403))

### FEATURE

- Remove `val_exec_ctx` feature flag
  ([#415](https://github.com/cosmos/ibc-rs/issues/415))

### IMPROVEMENTS

- Make all unit tests test the ValidationContext/ExecutionContext API
  ([#430](https://github.com/cosmos/ibc-rs/issues/430))
- Add an implementation of `validate_self_client` for the mock client
  ([#432](https://github.com/cosmos/ibc-rs/issues/432))
- Add a docstring and rename the `validate_self_client` argument for improved
  code documentation and readability
  ([#434](https://github.com/cosmos/ibc-rs/issues/434))
- Refactor connection handler unit tests to adapt with new Validation/Execution API
  ([#440](https://github.com/cosmos/ibc-rs/issues/440))

## v0.28.0

*February 9, 2023*

With this release, the implementation of the new `ValidationContext`/`ExecutionContext` is complete, although still behind the `val_exec_ctx` feature flag. There were also important bug fixes.

There are consensus-breaking changes.

### BREAKING CHANGES

- Implement `verify_upgrade_and_update_state` method for Tendermint clients
  ([#19](https://github.com/cosmos/ibc-rs/issues/19)).
- Remove support for asynchronous acknowledgements
  ([#361](https://github.com/cosmos/ibc-rs/issues/361))

### BUG FIXES

- Fix acknowledgement returned by the token transfer's onRecvPacket callback
  ([#369](https://github.com/cosmos/ibc-rs/issues/369))
- Mend `ChanOpenConfirm` handler check of expected counterparty state
  ([#396](https://github.com/cosmos/ibc-rs/issues/396))
- Fix issue with the error handling in the `new_check_header_and_update_state`
  method when consensus state is not found
  ([#405](https://github.com/cosmos/ibc-rs/issues/405))
- Fix the caught error by `get_packet_receipt` under `val_exec_ctx` feature when
  the packet receipt is not found
  ([#409](https://github.com/cosmos/ibc-rs/issues/409))

### FEATURE

- Finish implementing `ValidationContext::validate()` and
  `ExecutionContext::execute()` 
  ([#393](https://github.com/cosmos/ibc-rs/issues/393))

### IMPROVEMENTS

- Add tests to verify `AbciEvent` match the expected Ibc events
([#163](https://github.com/cosmos/ibc-rs/issues/163)).
- Add unit tests to cover edge scenarios for counterparty conn & chan ids at init phases
  ([#175](https://github.com/cosmos/ibc-rs/issues/175)).

## v0.27.0

*January 16, 2023*

This release contains a bug fix for the `ChanOpenConfirm` handler and it is strongly recommended to upgrade.

This release contains a consensus-breaking change during the channel opening handshake; it was broken, and now is fixed.

### BUG FIXES

- Fix ChanOpenConfirm handler check of counterparty state
  ([#353](https://github.com/cosmos/ibc-rs/issues/353))

## v0.26.0

*December 14, 2022*

This release contains miscellaneous improvements, focusing mainly on addressing technical debt.

There are no consensus-breaking changes.

### BREAKING CHANGES

- Exclude `ChannelEnd` from `MsgChannelOpenInit` and `MsgChannelOpenTry` and refactor their fields to match the spec
  ([#20](https://github.com/cosmos/ibc-rs/issues/20))
- Simplify Msg trait by removing unnecessary methods.
  ([#218](https://github.com/cosmos/ibc-rs/issues/218))
- Refactor proof handlers to conduct proof verifications inline with the process function 
  and apply naming conventions to packet messages types
  ([#230](https://github.com/cosmos/ibc-rs/issues/230))
- The function parameters in the Reader traits now references,
  while the functions in the Keeper traits take ownership directly.
  ([#304](https://github.com/cosmos/ibc-rs/issues/304))
- Change type of `trusted_validator_set` field in
  `MisbehaviourTrustedValidatorHashMismatch` error variant from `ValidatorSet` to
  `Vec<Validator>` to avoid clippy catches
  ([#309](https://github.com/cosmos/ibc-rs/issues/309))
- The function parameters in the `ValidationContext` trait now use references,
  while the functions in the `ExecutionContext` trait take ownership directly.
  ([#319](https://github.com/cosmos/ibc-rs/issues/319))
- Make internal `process()` `pub(crate)` 
  ([#338](https://github.com/cosmos/ibc-rs/issues/338))

### FEATURES

- Add serialization and deserialization features for codec and borsh to the host
  type in ics24 ([#259](https://github.com/cosmos/ibc-rs/issues/259))
- Add codec and borsh for ics04_channel::msgs::Acknowledgement and
  events::ModuleEvent ([#303](https://github.com/cosmos/ibc-rs/issues/303))
- Add parity-scale-codec and borsh for ibc::events::IbcEvent
  ([#320](https://github.com/cosmos/ibc-rs/issues/320))
- Make the code under mocks features work in no-std
  ([#311](https://github.com/cosmos/ibc-rs/issues/311))
- Make `serde` optional behind the `serde` feature flag
  ([#293](https://github.com/cosmos/ibc-rs/issues/293))

### IMPROVEMENTS

- Refactor unreachable test of conn_open_ack handler
  ([#30](https://github.com/cosmos/ibc-rs/issues/30))
- Remove legacy relayer-specific code and move ics18_relayer under the mock module
  ([#154](https://github.com/cosmos/ibc-rs/issues/154))
- Improve clippy catches and fix lint issues identified by clippy 0.1.67
  ([#309](https://github.com/cosmos/ibc-rs/issues/309))

## v0.25.0

*December 14, 2022*

This release updates the tendermint-rs dependency to v0.28.0 which includes important security improvements. Many other improvements have been made as well, including misbehaviour handling.

A lot of work has also been put towards implementing ADR 5, which is currently unfinished and has been put behind the feature flag `val_exec_ctx`.

The only consensus-breaking changes are the ones related to the fact that we now properly handle misbehaviour messages.

### BREAKING CHANGES

- Implement the IBC misbehaviour handler and misbehaviour handling logic for the Tendermint light client.
  ([#12](https://github.com/cosmos/ibc-rs/issues/12))
- `Ics20Context` no longer requires types implementing it to implement `ChannelReader` and `ChannelKeeper`, and instead depends on the `Ics20ChannelKeeper`
  and `SendPacketReader` traits. Additionally, the `send_packet` handler now requires the calling context to implement `SendPacketReader` and returns
  a `SendPacketResult`.
  ([#182](https://github.com/cosmos/ibc-rs/issues/182))
- Add `ValidationContext` and `ExecutionContext`, and implement for clients (ICS-2)
  ([#240](https://github.com/cosmos/ibc-rs/issues/240))
- Change `host_height`, `host_timestamp` return value to a `Result` in `ClientReader`, `ConnectionReader`, `ChannelReader` and `ValidationContext`
  ([#242](https://github.com/cosmos/ibc-rs/issues/242))
- Rename Ics* names to something more descriptive
  ([#245](https://github.com/cosmos/ibc-rs/issues/245))
- Implement `ValidationContext::validate` and `ExecutionContext::execute` for connections (ICS-3)
  ([#251](https://github.com/cosmos/ibc-rs/issues/251))
- Implement misbehaviour in `ExecutionContext` and `ValidationContext`
  ([#281](https://github.com/cosmos/ibc-rs/issues/281))
- Update `tendermint` dependencies to `v0.28.0`, which contain an important security fix.
([#294](https://github.com/cosmos/ibc-rs/issues/294))

### BUG FIXES

- Set counterparty connection ID to None in `conn_open_init` ([#174](https://github.com/cosmos/ibc-rs/issues/174))
- Verify the message's counterparty connection ID in `conn_open_ack`
  instead of the store's ([#274](https://github.com/cosmos/ibc-rs/issues/274))

### IMPROVEMENTS

- Remove `flex-error` and remove unused error variants([#164](https://github.com/cosmos/ibc-rs/issues/164))
- ConnectionMsg::ConnectionOpen{Try, Ack} should not wrap a Box
  ([#258](https://github.com/cosmos/ibc-rs/issues/258))
- Track code coverage with `cargo-llvm-cov`
  ([#277](https://github.com/cosmos/ibc-rs/issues/277))

## v0.24.0

*December 8, 2022*

This release mainly updates the tendermint-rs dependency to v0.27.0 and includes security improvements.

There are no consensus-breaking changes.

### BREAKING CHANGES

- Update to changes in tendermint-rs 0.27
  ([#260](https://github.com/cosmos/ibc-rs/pulls/260))

### IMPROVEMENTS

- Update `ics23` to v0.9.0, which contains substantial security improvements
  ([#278](https://github.com/cosmos/ibc-rs/issues/278))

## v0.23.0

*November 21, 2022*

This release mainly updates the tendermint-rs dependency to v0.26.0.

There are no consensus-breaking changes.

### BREAKING CHANGES

- Update to tendermint-rs 0.26 and ibc-proto 0.22
  ([#208](https://github.com/cosmos/ibc-rs/issues/208))

### FEATURES

- Add Other Item for Ics02Client,Ics03connection, Ics04Channel Error
  ([#237](https://github.com/cosmos/ibc-rs/issues/237))

## v0.22.0

*November 9, 2022*

This release includes major improvements in making the library compatible with ibc-go v5.0.1. This includes making ibc events compatible and removing the crossing-hellos logic from the connection and channel handshakes.

There are consensus-breaking changes in the connection and channel handshakes. However, there are no consensus-breaking changes for already established channels.

### BREAKING CHANGES

- Make connection events compatible with ibc-go
  ([#145](https://github.com/cosmos/ibc-rs/issues/145))
- Makes channel/packet events compatible with ibc-go
  ([#146](https://github.com/cosmos/ibc-rs/issues/146))
- Remove crossing hellos logic from connection handshake. Breaking changes in 
  connection message types.
  ([#156](https://github.com/cosmos/ibc-rs/issues/156)).
- Remove crossing hellos logic from channel handshake
  ([#157](https://github.com/cosmos/ibc-rs/issues/157))
- call `validate_self_client` in `conn_open_try` and `conn_open_ack`,
  and provide a tendermint implementation for `validate_self_client`
  ([#162](https://github.com/cosmos/ibc-rs/issues/162))
- Refactor channel handlers. Proof calls were inlined, and our handshake
  variable naming convention was applied
  ([#166](https://github.com/cosmos/ibc-rs/issues/166))
- Change `ClientType` to contain a `String` instead of `&'static str`
  ([#206](https://github.com/cosmos/ibc-rs/issues/206))

### BUG FIXES

- Connection consensus state proof verification now properly uses `consensus_height`
  ([#168](https://github.com/cosmos/ibc-rs/issues/168)).
- Allow one-letter chain names in `ChainId::is_epoch_format`
  ([#211](https://github.com/cosmos/ibc-rs/issues/211))
- Don't panic on user input in channel proof verification
  ([#219](https://github.com/cosmos/ibc-rs/issues/219))

### FEATURES

- Add getter functions to SendPacket, ReceivePacket, WriteAcknowledgement,
  AcknowledgePacket, TimeoutPacket to get the elements of the structure
  ([#231](https://github.com/cosmos/ibc-rs/issues/231))

## v0.21.1

*October 27, 2022*

This release fixes a critical vulnerability. It is strongly advised to upgrade.

### BUG FIXES

- No longer panic when packet data is not valid UTF-8
  ([#199](https://github.com/cosmos/ibc-rs/issues/199))

## v0.21.0

*October 24, 2022*

This is a small release that allows new `ClientTypes` to be created, which was missed when implementing ADR 4. The changes are not consensus-breaking.

### BREAKING CHANGES

- Make ClientType allow any string value as opposed to just Tendermint
  ([#188](https://github.com/cosmos/ibc-rs/issues/188))

## v0.20.0

*October 19, 2022*

This is a major release, which implemented [ADR 4](https://github.com/cosmos/ibc-rs/blob/main/docs/architecture/adr-004-light-client-crates-extraction.md), as well as some miscellaneous bug fixes. Please see the corresponding sections for more information.

### BREAKING CHANGES

- Add missing Tendermint `ClientState` checks and make all its fields private.
- Add a `frozen_height` input parameter to `ClientState::new()`.
  ([#22](https://github.com/cosmos/ibc-rs/issues/22)).
- Remove `Display` from `IbcEvent` ([#144](https://github.com/cosmos/ibc-rs/issues/144)).
- Remove `IbcEvent::Empty` ([#144](https://github.com/cosmos/ibc-rs/issues/144)).
- Make `client_state` field required in `MsgConnectionOpenTry` and
  `MsgConnectionOpenAck`. Necessary for correctness according to spec.  
  ([#159](https://github.com/cosmos/ibc-rs/issues/159)).
- Redesign the API to allow light client implementations to be hosted outside the ibc-rs repository. 
  ([#2483](https://github.com/informalsystems/ibc-rs/pull/2483)).

### BUG FIXES

- Make client events compatible with ibc-go v5
  ([#144](https://github.com/cosmos/ibc-rs/issues/144)).
- Delete packet commitment in acknowledge packet handler regardless of channel ordering
  ([#2229](https://github.com/informalsystems/ibc-rs/issues/2229)).

### FEATURES

- Public PrefixedDenom inner type and add as_str func for BaseDenom 
  ([#161](https://github.com/cosmos/ibc-rs/issues/161))

### IMPROVEMENTS

- Derive Hash for ModuleId ([#179](https://github.com/cosmos/ibc-rs/issues/179))
- Improved `core::ics04_channel` APIs, avoiding poor ergonomics of
  reference-to-tuple arguments and inconsistent ownership patterns.
  ([#2603](https://github.com/informalsystems/ibc-rs/pull/2603)).

### DESIGN DECISIONS
- Propose ADR05 for handlers validation and execution separation.
  ([#2582](https://github.com/informalsystems/ibc-rs/pull/2582)).

## v0.19.0

*August 22nd, 2022*

#### BREAKING CHANGES

- Remove `height` attribute from `IbcEvent` and its variants
  ([#2542](https://github.com/informalsystems/ibc-rs/issues/2542))

#### BUG FIXES

- Fix `MsgTimeoutOnClose` to verify the channel proof
  ([#2534](https://github.com/informalsystems/ibc-rs/issues/2534))


## v0.18.0

*August 8th, 2022*

#### IMPROVEMENTS

- Remove Deserialize from IbcEvent and variants
  ([#2481](https://github.com/informalsystems/ibc-rs/issues/2481))


## v0.17.0

*July 27th, 2022*

#### BREAKING CHANGES

- Remove provided `Ics20Reader::get_channel_escrow_address()` implementation and make `cosmos_adr028_escrow_address()` public.
  ([#2387](https://github.com/informalsystems/ibc-rs/issues/2387))

#### BUG FIXES

- Fix serialization for ICS20 packet data structures
  ([#2386](https://github.com/informalsystems/ibc-rs/issues/2386))
- Properly process `WriteAcknowledgement`s on packet callback
  ([#2424](https://github.com/informalsystems/ibc-rs/issues/2424))
- Fix `write_acknowledgement` handler which incorrectly used packet's `source_{port, channel}` as key for storing acks
  ([#2428](https://github.com/informalsystems/ibc-rs/issues/2428))

#### IMPROVEMENTS

- Propose ADR011 for light client extraction
  ([#2356](https://github.com/informalsystems/ibc-rs/pull/2356))


## v0.16.0

*July 7th, 2022*

#### BREAKING CHANGES

- Change `ChannelId` representation to a string, allowing all IDs valid per ICS 024
  ([#2330](https://github.com/informalsystems/ibc-rs/issues/2330)).

#### BUG FIXES

- Fix `recv_packet` handler incorrectly querying `packet_receipt` and `next_sequence_recv` using
  packet's `source_{port, channel}`.
  ([#2293](https://github.com/informalsystems/ibc-rs/issues/2293))
- Permit channel identifiers with length up to 64 characters,
  as per the ICS 024 specification.
  ([#2330](https://github.com/informalsystems/ibc-rs/issues/2330)).

#### IMPROVEMENTS

- Remove the concept of a zero Height
  ([#1009](https://github.com/informalsystems/ibc-rs/issues/1009))
- Complete ICS20 implementation ([#1759](https://github.com/informalsystems/ibc-rs/issues/1759))
- Derive `serde::{Serialize, Deserialize}` for `U256`. ([#2279](https://github.com/informalsystems/ibc-rs/issues/2279))
- Remove unnecessary supertraits requirements from ICS20 traits.
  ([#2280](https://github.com/informalsystems/ibc-rs/pull/2280))


## v0.15.0

*May 23rd, 2022*

### BUG FIXES

- Fix packet commitment calculation to match ibc-go
  ([#2104](https://github.com/informalsystems/ibc-rs/issues/2104))
- Fix incorrect acknowledgement verification
  ([#2114](https://github.com/informalsystems/ibc-rs/issues/2114))
- fix connection id mix-up in connection acknowledgement processing
  ([#2178](https://github.com/informalsystems/ibc-rs/issues/2178))

### IMPROVEMENTS

- Remove object capabilities from the modules
  ([#2159](https://github.com/informalsystems/ibc-rs/issues/2159))


## v0.14.1

*May 2nd, 2022*

> This is a legacy version with no ibc crate changes. 

## v0.14.0

*April 27th, 2022*

### BUG FIXES

- Make all handlers emit an IbcEvent with current host chain height as height parameter value.
  ([#2035](https://github.com/informalsystems/ibc-rs/issues/2035))
- Use the version in the message when handling a MsgConnOpenInit
  ([#2062](https://github.com/informalsystems/ibc-rs/issues/2062))

### IMPROVEMENTS

- Complete ICS26 implementation ([#1758](https://github.com/informalsystems/ibc-rs/issues/1758))
- Improve `ChannelId` validation. ([#2068](https://github.com/informalsystems/ibc-rs/issues/2068))


## v0.13.0
*March 28th, 2022*

### IMPROVEMENTS

- Refactored channels events in ICS 04 module
  ([#718](https://github.com/informalsystems/ibc-rs/issues/718))


## v0.12.0
*February 24th, 2022*

### BUG FIXES

- Fixed the formatting of NotEnoughTimeElapsed and NotEnoughBlocksElapsed
  in Tendermint errors ([#1706](https://github.com/informalsystems/ibc-rs/issues/1706))
- IBC handlers now retrieve the host timestamp from the latest host consensus
  state ([#1770](https://github.com/informalsystems/ibc-rs/issues/1770))

### IMPROVEMENTS

- Added more unit tests to verify Tendermint ClientState
  ([#1706](https://github.com/informalsystems/ibc-rs/issues/1706))
- Define CapabilityReader and CapabilityKeeper traits
  ([#1769](https://github.com/informalsystems/ibc-rs/issues/1769))
- [Relayer Library](https://github.com/informalsystems/hermes/tree/master/crates/relayer)
  - Add two more health checks: tx indexing enabled and historical entries > 0
    ([#1388](https://github.com/informalsystems/ibc-rs/issues/1388))
  - Changed `ConnectionEnd::versions` method to be non-allocating by having it return a `&[Version]` instead of `Vec<Version>`
    ([#1880](https://github.com/informalsystems/ibc-rs/pull/1880))

## v0.11.1
*February 4th, 2022*

> This is a legacy version with no ibc crate changes.


## v0.11.0
*January 27th, 2022*

### BREAKING CHANGES

- Hide `ibc::Timestamp::now()` behind `clock` feature flag ([#1612](https://github.com/informalsystems/ibc-rs/issues/1612))

### BUG FIXES

- Verify the client consensus proof against the client's consensus state root and not the host's state root
  [#1745](https://github.com/informalsystems/ibc-rs/issues/1745)
- Initialize consensus metadata on client creation
  ([#1763](https://github.com/informalsystems/ibc-rs/issues/1763))

### IMPROVEMENTS

  - Extract all `ics24_host::Path` variants into their separate types
    ([#1760](https://github.com/informalsystems/ibc-rs/issues/1760))
  - Disallow empty `CommitmentPrefix` and `CommitmentProofBytes`
    ([#1761](https://github.com/informalsystems/ibc-rs/issues/1761))

## v0.10.0
*January 13th, 2021*

### BREAKING CHANGES

- Add the `frozen_height()` method to the `ClientState` trait (includes breaking changes to the Tendermint `ClientState` API).
  ([#1618](https://github.com/informalsystems/ibc-rs/issues/1618))
- Remove `Timestamp` API that depended on the `chrono` crate:
  ([#1665](https://github.com/informalsystems/ibc-rs/pull/1665)):
  - `Timestamp::from_datetime`; use `From<tendermint::Time>`
  - `Timestamp::as_datetime`, superseded by `Timestamp::into_datetime`

### BUG FIXES

- Delete packet commitment instead of acknowledgement in acknowledgePacket
  [#1573](https://github.com/informalsystems/ibc-rs/issues/1573)
- Set the `counterparty_channel_id` correctly to fix ICS04 [`chanOpenAck` handler verification](https://github.com/informalsystems/ibc-rs/blob/master/modules/src/core/ics04_channel/handler/chan_open_ack.rs)
  ([#1649](https://github.com/informalsystems/ibc-rs/issues/1649))
- Add missing assertion for non-zero trust-level in Tendermint client initialization.
  ([#1697](https://github.com/informalsystems/ibc-rs/issues/1697))
- Fix conversion to Protocol Buffers of `ClientState`'s `frozen_height` field.
  ([#1710](https://github.com/informalsystems/ibc-rs/issues/1710))

### FEATURES

- Implement proof verification for Tendermint client (ICS07).
  ([#1583](https://github.com/informalsystems/ibc-rs/pull/1583))

### IMPROVEMENTS

- More conventional ad-hoc conversion methods on `Timestamp`
  ([#1665](https://github.com/informalsystems/ibc-rs/pull/1665)):
- `Timestamp::nanoseconds` replaces `Timestamp::as_nanoseconds`
- `Timestamp::into_datetime` substitutes `Timestamp::as_datetime`

## v0.9.0, the “Zamfir” release
*November 23rd, 2021*

### BUG FIXES

- Set the connection counterparty in the ICS 003 [`connOpenAck` handler][conn-open-ack-handler]
  ([#1532](https://github.com/informalsystems/ibc-rs/issues/1532))

[conn-open-ack-handler]: https://github.com/informalsystems/ibc-rs/blob/master/modules/src/core/ics03_connection/handler/conn_open_ack.rs

### IMPROVEMENTS

- Derive `PartialEq` and `Eq` on `IbcEvent` and inner types
  ([#1546](https://github.com/informalsystems/ibc-rs/issues/1546))


## v0.8.0
*October 29th, 2021*

### IMPROVEMENTS

- Support for converting `ibc::events::IbcEvent` into `tendermint::abci::Event`
  ([#838](https://github.com/informalsystems/ibc-rs/issues/838))
- Restructure the layout of the `ibc` crate to match `ibc-go`'s [layout](https://github.com/cosmos/ibc-go#contents)
  ([#1436](https://github.com/informalsystems/ibc-rs/issues/1436))
- Implement `FromStr<Path>` to enable string-encoded paths to be converted into Path identifiers
  ([#1460](https://github.com/informalsystems/ibc-rs/issues/1460))


## v0.8.0-pre.1
*October 22nd, 2021*

### BREAKING CHANGES

- The `check_header_and_update_state` method of the `ClientDef`
  trait (ICS02) has been expanded to facilitate ICS07
  ([#1214](https://github.com/informalsystems/ibc-rs/issues/1214))

### FEATURES

- Add ICS07 verification functionality by using `tendermint-light-client`
  ([#1214](https://github.com/informalsystems/ibc-rs/issues/1214))


## v0.7.3
*October 4th, 2021*

> This is a legacy version with no ibc crate changes.


## v0.7.2
*September 24th, 2021*


## v0.7.1
*September 14th, 2021*

### IMPROVEMENTS

- Change all `*Reader` traits to return `Result` instead of `Option` ([#1268])
- Clean up modules' errors ([#1333])

[#1268]: https://github.com/informalsystems/ibc-rs/issues/1268
[#1333]: https://github.com/informalsystems/ibc-rs/issues/1333


## v0.7.0
*August 24th, 2021*

### BUG FIXES

- Set the index of `ibc::ics05_port::capabilities::Capability` ([#1257])

[#1257]: https://github.com/informalsystems/ibc-rs/issues/1257

### IMPROVEMENTS

- Implement `ics02_client::client_consensus::ConsensusState` for `AnyConsensusState` ([#1297])

[#1297]: https://github.com/informalsystems/ibc-rs/issues/1297


## v0.6.2
*August 2nd, 2021*

### BUG FIXES

- Add missing `Protobuf` impl for `ics03_connection::connection::Counterparty` ([#1247])

[#1247]: https://github.com/informalsystems/ibc-rs/issues/1247

### FEATURES

- Use the [`flex-error`](https://docs.rs/flex-error/) crate to define and
handle errors ([#1158])


## v0.6.1
*July 22nd, 2021*

### FEATURES

- Enable `pub` access to verification methods of ICS 03 & 04 ([#1198])
- Add `ics26_routing::handler::decode` function ([#1194])
- Add a pseudo root to `MockConsensusState` ([#1215])

### BUG FIXES

- Fix stack overflow in `MockHeader` implementation ([#1192])
- Align `as_str` and `from_str` behavior in `ClientType` ([#1192])

[#1192]: https://github.com/informalsystems/ibc-rs/issues/1192
[#1194]: https://github.com/informalsystems/ibc-rs/issues/1194
[#1198]: https://github.com/informalsystems/ibc-rs/issues/1198
[#1215]: https://github.com/informalsystems/ibc-rs/issues/1215


## v0.6.0
*July 12th, 2021*

> This is a legacy version with no ibc crate changes.


## v0.5.0
*June 22nd, 2021*

> This is a legacy version with no ibc crate changes.


## v0.4.0
*June 3rd, 2021*

### IMPROVEMENTS

- Started `unwrap` cleanup ([#871])

[#871]: https://github.com/informalsystems/ibc-rs/issues/871


## v0.3.2
*May 21st, 2021*

> This is a legacy version with no ibc crate changes.


## v0.3.1
*May 14h, 2021*

### BUG FIXES

- Process raw `delay_period` field as nanoseconds instead of seconds. ([#927])

[#927]: https://github.com/informalsystems/ibc-rs/issues/927


## v0.3.0
*May 7h, 2021*

### IMPROVEMENTS

- Reinstated `ics23` dependency ([#854])
- Use proper Timestamp type to track time ([#758])

### BUG FIXES

- Fix parsing in `chain_version` when chain identifier has multiple dashes ([#878])

[#758]: https://github.com/informalsystems/ibc-rs/issues/758
[#854]: https://github.com/informalsystems/ibc-rs/issues/854
[#878]: https://github.com/informalsystems/ibc-rs/issues/878


## v0.2.0
*April 14th, 2021*

### FEATURES

- Added handler(s) for sending packets ([#695]), recv. and ack. packets ([#736]), and timeouts ([#362])

### IMPROVEMENTS

- Follow Rust guidelines naming conventions ([#689])
- Per client structure modules ([#740])
- MBT: use modelator crate ([#761])

### BUG FIXES

- Fix overflow bug in ICS03 client consensus height verification method ([#685])
- Allow a conn open ack to succeed in the happy case ([#699])

### BREAKING CHANGES

- `MsgConnectionOpenAck.counterparty_connection_id` is now a `ConnectionId` instead of an `Option<ConnectionId>`([#700])

[#362]: https://github.com/informalsystems/ibc-rs/issues/362
[#685]: https://github.com/informalsystems/ibc-rs/issues/685
[#689]: https://github.com/informalsystems/ibc-rs/issues/689
[#699]: https://github.com/informalsystems/ibc-rs/issues/699
[#700]: https://github.com/informalsystems/ibc-rs/pull/700
[#736]: https://github.com/informalsystems/ibc-rs/issues/736
[#740]: https://github.com/informalsystems/ibc-rs/issues/740
[#761]: https://github.com/informalsystems/ibc-rs/issues/761


## v0.1.1
*February 17, 2021*

### IMPROVEMENTS

- Change event height to ICS height ([#549])

### BUG FIXES

- Fix panic in conn open try when no connection id is provided ([#626])
- Disable MBT tests if the "mocks" feature is not enabled ([#643])

### BREAKING CHANGES

- Implementation of the `ChanOpenAck`, `ChanOpenConfirm`, `ChanCloseInit`, and `ChanCloseConfirm` handlers ([#316])
- Remove dependency on `tendermint-rpc` ([#624])

[#316]: https://github.com/informalsystems/ibc-rs/issues/316
[#549]: https://github.com/informalsystems/ibc-rs/issues/549
[#624]: https://github.com/informalsystems/ibc-rs/issues/624
[#626]: https://github.com/informalsystems/ibc-rs/issues/626
[#643]: https://github.com/informalsystems/ibc-rs/issues/643


## v0.1.0
*February 4, 2021*

### FEATURES

- Add `MsgTimeoutOnClose` message type ([#563])
- Implement `MsgChannelOpenTry` message handler ([#543])

### IMPROVEMENTS

- Clean the `validate_basic` method ([#94])
- `MsgConnectionOpenAck` testing improvements ([#306])

### BUG FIXES:

- Fix for storing `ClientType` upon 'create-client' ([#513])

### BREAKING CHANGES:

- The `ibc::handler::Event` is removed and handlers now produce `ibc::events::IBCEvent`s ([#535])

[#94]: https://github.com/informalsystems/ibc-rs/issues/94
[#306]: https://github.com/informalsystems/ibc-rs/issues/306
[#513]: https://github.com/informalsystems/ibc-rs/issues/513
[#535]: https://github.com/informalsystems/ibc-rs/issues/535
[#543]: https://github.com/informalsystems/ibc-rs/issues/543
[#563]: https://github.com/informalsystems/ibc-rs/issues/563


## v0.0.6
*December 23, 2020*

> This is a legacy version with no ibc crate changes.


## v0.0.5
*December 2, 2020*

### FEATURES

- Implement flexible connection id selection ([#332])
- ICS 4 Domain Types for channel handshakes and packets ([#315], [#95])
- Introduce LightBlock support for MockContext ([#389])


### IMPROVEMENTS

- Split `msgs.rs` of ICS002 in separate modules ([#367])
- Fixed inconsistent versioning for ICS003 and ICS004 ([#97])
- Fixed `get_sign_bytes` method for messages ([#98])
- Homogenize ConnectionReader trait so that all functions return owned objects ([#347])
- Align with tendermint-rs in the domain type definition of `block::Id` ([#338])


[#95]: https://github.com/informalsystems/ibc-rs/issues/95
[#97]: https://github.com/informalsystems/ibc-rs/issues/97
[#98]: https://github.com/informalsystems/ibc-rs/issues/98
[#332]: https://github.com/informalsystems/ibc-rs/issues/332
[#338]: https://github.com/informalsystems/ibc-rs/issues/338
[#347]: https://github.com/informalsystems/ibc-rs/issues/347
[#367]: https://github.com/informalsystems/ibc-rs/issues/367
[#368]: https://github.com/informalsystems/ibc-rs/issues/368
[#389]: https://github.com/informalsystems/ibc-rs/issues/389


## v0.0.4
*October 19, 2020*

### FEATURES:
- ICS03 Ack and Confirm message processors ([#223])
- Routing module minimal implementation for MVP ([#159], [#232])
- Basic relayer functionality: a test with ClientUpdate ping-pong between two mocked chains ([#276])

### IMPROVEMENTS:
- Implemented the `DomainType` trait for IBC proto structures ([#245], [#249]).
- ICS03 connection handshake protocol initial implementation and tests ([#160])
- Add capability to decode from protobuf Any* type into Tendermint and Mock client states
- Cleanup Any* client wrappers related code
- Migrate handlers to newer protobuf definitions ([#226])
- Extend client context mock ([#221])
- Context mock simplifications and cleanup ([#269], [#295], [#296], [#297])
- Split `msgs.rs` in multiple files, implement `From` for all messages ([#253])

### BUG FIXES:
- Removed "Uninitialized" state from connection ([#217])
- Disclosed bugs in ICS3 version negotiation and proposed a fix ([#209], [#213])

[#159]: https://github.com/informalsystems/ibc-rs/issues/159
[#160]: https://github.com/informalsystems/ibc-rs/issues/160
[#207]: https://github.com/informalsystems/ibc-rs/issues/207
[#209]: https://github.com/informalsystems/ibc-rs/issues/209
[#213]: https://github.com/informalsystems/ibc-rs/issues/213
[#217]: https://github.com/informalsystems/ibc-rs/issues/217
[#221]: https://github.com/informalsystems/ibc-rs/issues/221
[#223]: https://github.com/informalsystems/ibc-rs/issues/223
[#226]: https://github.com/informalsystems/ibc-rs/issues/226
[#232]: https://github.com/informalsystems/ibc-rs/issues/232
[#245]: https://github.com/informalsystems/ibc-rs/issues/245
[#249]: https://github.com/informalsystems/ibc-rs/issues/249
[#269]: https://github.com/informalsystems/ibc-rs/issues/269
[#276]: https://github.com/informalsystems/ibc-rs/issues/276
[#295]: https://github.com/informalsystems/ibc-rs/issues/295
[#296]: https://github.com/informalsystems/ibc-rs/issues/296
[#297]: https://github.com/informalsystems/ibc-rs/issues/297


## v0.0.3
*September 1, 2020*

### BREAKING CHANGES:
- Renamed `modules` crate to `ibc` crate. Version number for the new crate is not reset. ([#198])
- `ConnectionId`s are now decoded to `Vec<ConnectionId>` and validated instead of `Vec<String>` ([#185])
- Removed `Connection` and `ConnectionCounterparty` traits ([#193])
- Removed `Channel` and `ChannelCounterparty` traits ([#192])

### FEATURES:
- partial implementation of message handler ([#119], [#194])
- partial implementation of message handler ([#119], [#194])
- Proposal for IBC handler (message processor) architecture ([#119], [#194])
- Documentation for the repository structure ([#1])
- Connection Handshake FSM English description ([#122])

### BUG FIXES:
- Identifiers limit update according to ICS specs ([#168])

[#1]: https://github.com/informalsystems/ibc-rs/issues/1
[#119]: https://github.com/informalsystems/ibc-rs/issues/119
[#122]: https://github.com/informalsystems/ibc-rs/issues/122
[#168]: https://github.com/informalsystems/ibc-rs/issues/168
[#185]: https://github.com/informalsystems/ibc-rs/issues/185
[#192]: https://github.com/informalsystems/ibc-rs/issues/192
[#193]: https://github.com/informalsystems/ibc-rs/issues/193
[#194]: https://github.com/informalsystems/ibc-rs/issues/194
[#198]: https://github.com/informalsystems/ibc-rs/issues/198


## v0.0.2

*August 1, 2020*

### BREAKING CHANGES:

- Refactor queries, paths, and Chain trait to reduce code and use
  protobuf instead of Amino.
        [\#152](https://github.com/informalsystems/ibc-rs/pull/152),
        [\#174](https://github.com/informalsystems/ibc-rs/pull/174),
        [\#155](https://github.com/informalsystems/ibc-rs/pull/155)

### FEATURES:

- Channel closing datagrams in TLA+ [\#141](https://github.com/informalsystems/ibc-rs/pull/141)

### IMPROVEMENTS:

- Implemented better Raw type handling. [\#156](https://github.com/informalsystems/ibc-rs/pull/156)

### BUG FIXES:

- Fixed the identifiers limits according to updated ics spec. [\#189](https://github.com/informalsystems/ibc-rs/pull/189)
- Fix nightly runs. [\#161](https://github.com/informalsystems/ibc-rs/pull/161)
- Fix for incomplete licence terms. [\#153](https://github.com/informalsystems/ibc-rs/pull/153)


## 0.0.1

*July 1st, 2020*

This is the initial prototype release of an IBC relayer and TLA+ specifications.
There are no compatibility guarantees until v0.1.0.

Includes:

- Client state, consensus state, connection, channel queries.
    - Note: deserialization is unimplemented as it has dependency on migration to protobuf for ABCI queries
- IBC Modules partial implementation for datastructures, messages and queries.
- Some English and TLA+ specifications for Connection & Channel Handshake as well as naive relayer algorithm.
