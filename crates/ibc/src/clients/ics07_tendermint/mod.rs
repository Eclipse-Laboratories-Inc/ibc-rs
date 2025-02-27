//! ICS 07: Tendermint Client implements a client verification algorithm for blockchains which use
//! the Tendermint consensus algorithm.

use alloc::string::ToString;

use crate::core::ics02_client::client_type::ClientType;

pub mod client_state;
pub mod consensus_state;
pub mod error;
pub mod header;
pub mod misbehaviour;
pub mod trust_threshold;

pub(crate) const TENDERMINT_CLIENT_TYPE: &str = "07-tendermint";

/// Returns the tendermint `ClientType`
pub fn client_type() -> ClientType {
    ClientType::from(TENDERMINT_CLIENT_TYPE.to_string())
}
