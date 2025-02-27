use crate::prelude::*;

use crate::core::ics02_client::client_state::ClientState;
use crate::core::ics02_client::header::Header;

use crate::core::ics24_host::identifier::ClientId;
use crate::core::ContextError;
use crate::signer::Signer;
use crate::Height;

/// Trait capturing all dependencies (i.e., the context) which algorithms in ICS18 require to
/// relay packets between chains. This trait comprises the dependencies towards a single chain.
/// Most of the functions in this represent wrappers over the ABCI interface.
/// This trait mimics the `Chain` trait, but at a lower level of abstraction (no networking, header
/// types, light client, RPC client, etc.)
pub trait RelayerContext {
    /// Returns the latest height of the chain.
    fn query_latest_height(&self) -> Result<Height, ContextError>;

    /// Returns this client state for the given `client_id` on this chain.
    /// Wrapper over the `/abci_query?path=..` endpoint.
    fn query_client_full_state(&self, client_id: &ClientId) -> Option<Box<dyn ClientState>>;

    /// Returns the most advanced header of this chain.
    fn query_latest_header(&self) -> Option<Box<dyn Header>>;

    /// Temporary solution. Similar to `CosmosSDKChain::key_and_signer()` but simpler.
    fn signer(&self) -> Signer;
}

#[cfg(test)]
mod tests {
    use crate::clients::ics07_tendermint::client_type as tm_client_type;
    use crate::core::ics02_client::header::Header;
    use crate::core::ics02_client::msgs::update_client::MsgUpdateClient;
    use crate::core::ics02_client::msgs::ClientMsg;
    use crate::core::ics24_host::identifier::{ChainId, ClientId};
    use crate::core::MsgEnvelope;
    use crate::mock::client_state::client_type as mock_client_type;
    use crate::mock::context::MockContext;
    use crate::mock::host::{HostBlock, HostType};
    use crate::mock::ics18_relayer::context::RelayerContext;
    use crate::mock::ics18_relayer::error::RelayerError;
    use crate::prelude::*;
    use crate::Height;

    use test_log::test;
    use tracing::debug;

    fn downcast_header<H: Header>(h: &dyn Header) -> Option<&H> {
        h.as_any().downcast_ref::<H>()
    }

    /// Builds a `ClientMsg::UpdateClient` for a client with id `client_id` running on the `dest`
    /// context, assuming that the latest header on the source context is `src_header`.
    pub(crate) fn build_client_update_datagram<Ctx>(
        dest: &Ctx,
        client_id: &ClientId,
        src_header: &dyn Header,
    ) -> Result<ClientMsg, RelayerError>
    where
        Ctx: RelayerContext,
    {
        // Check if client for ibc0 on ibc1 has been updated to latest height:
        // - query client state on destination chain
        let dest_client_state = dest.query_client_full_state(client_id).ok_or_else(|| {
            RelayerError::ClientStateNotFound {
                client_id: client_id.clone(),
            }
        })?;

        let dest_client_latest_height = dest_client_state.latest_height();

        if src_header.height() == dest_client_latest_height {
            return Err(RelayerError::ClientAlreadyUpToDate {
                client_id: client_id.clone(),
                source_height: src_header.height(),
                destination_height: dest_client_latest_height,
            });
        };

        if dest_client_latest_height > src_header.height() {
            return Err(RelayerError::ClientAtHigherHeight {
                client_id: client_id.clone(),
                source_height: src_header.height(),
                destination_height: dest_client_latest_height,
            });
        };

        // Client on destination chain can be updated.
        Ok(ClientMsg::UpdateClient(MsgUpdateClient {
            client_id: client_id.clone(),
            header: src_header.clone_into(),
            signer: dest.signer(),
        }))
    }

    #[test]
    /// Serves to test both ICS 26 `dispatch` & `build_client_update_datagram` functions.
    /// Implements a "ping pong" of client update messages, so that two chains repeatedly
    /// process a client update message and update their height in succession.
    fn client_update_ping_pong() {
        let chain_a_start_height = Height::new(1, 11).unwrap();
        let chain_b_start_height = Height::new(1, 20).unwrap();
        let client_on_b_for_a_height = Height::new(1, 10).unwrap(); // Should be smaller than `chain_a_start_height`
        let client_on_a_for_b_height = Height::new(1, 20).unwrap(); // Should be smaller than `chain_b_start_height`
        let num_iterations = 4;

        let client_on_a_for_b = ClientId::new(tm_client_type(), 0).unwrap();
        let client_on_b_for_a = ClientId::new(mock_client_type(), 0).unwrap();

        let chain_id_a = ChainId::new("mockgaiaA".to_string(), 1);
        let chain_id_b = ChainId::new("mockgaiaB".to_string(), 1);

        // Create two mock contexts, one for each chain.
        let mut ctx_a =
            MockContext::new(chain_id_a.clone(), HostType::Mock, 5, chain_a_start_height)
                .with_client_parametrized_with_chain_id(
                    chain_id_b.clone(),
                    &client_on_a_for_b,
                    client_on_a_for_b_height,
                    Some(tm_client_type()), // The target host chain (B) is synthetic TM.
                    Some(client_on_a_for_b_height),
                );
        let mut ctx_b = MockContext::new(
            chain_id_b,
            HostType::SyntheticTendermint,
            5,
            chain_b_start_height,
        )
        .with_client_parametrized_with_chain_id(
            chain_id_a,
            &client_on_b_for_a,
            client_on_b_for_a_height,
            Some(mock_client_type()), // The target host chain is mock.
            Some(client_on_b_for_a_height),
        );

        for _i in 0..num_iterations {
            // Update client on chain B to latest height of A.
            // - create the client update message with the latest header from A
            let a_latest_header = ctx_a.query_latest_header().unwrap();
            let client_msg_b_res =
                build_client_update_datagram(&ctx_b, &client_on_b_for_a, a_latest_header.as_ref());

            assert!(
                client_msg_b_res.is_ok(),
                "create_client_update failed for context destination {ctx_b:?}, error: {client_msg_b_res:?}",                
            );

            let client_msg_b = client_msg_b_res.unwrap();

            // - send the message to B. We bypass ICS18 interface and call directly into
            // MockContext `recv` method (to avoid additional serialization steps).
            let dispatch_res_b = ctx_b.deliver(MsgEnvelope::Client(client_msg_b));
            let validation_res = ctx_b.validate();
            assert!(
                validation_res.is_ok(),
                "context validation failed with error {validation_res:?} for context {ctx_b:?}",
            );

            // Check if the update succeeded.
            assert!(
                dispatch_res_b.is_ok(),
                "Dispatch failed for host chain b with error: {dispatch_res_b:?}"
            );
            let client_height_b = ctx_b
                .query_client_full_state(&client_on_b_for_a)
                .unwrap()
                .latest_height();
            assert_eq!(client_height_b, ctx_a.query_latest_height().unwrap());

            // Update client on chain A to latest height of B.
            // - create the client update message with the latest header from B
            // The test uses LightClientBlock that does not store the trusted height
            let b_latest_header = ctx_b.query_latest_header().unwrap();
            let b_latest_header: &HostBlock = downcast_header(b_latest_header.as_ref()).unwrap();
            let mut b_latest_header = b_latest_header.clone();

            let th = b_latest_header.height();
            b_latest_header.set_trusted_height(th.decrement().unwrap());

            let client_msg_a_res = build_client_update_datagram(
                &ctx_a,
                &client_on_a_for_b,
                b_latest_header.into_box().as_ref(),
            );

            assert!(
                client_msg_a_res.is_ok(),
                "create_client_update failed for context destination {ctx_a:?}, error: {client_msg_a_res:?}",
            );

            let client_msg_a = client_msg_a_res.unwrap();

            debug!("client_msg_a = {:?}", client_msg_a);

            // - send the message to A
            let dispatch_res_a = ctx_a.deliver(MsgEnvelope::Client(client_msg_a));
            let validation_res = ctx_a.validate();
            assert!(
                validation_res.is_ok(),
                "context validation failed with error {validation_res:?} for context {ctx_a:?}",
            );

            // Check if the update succeeded.
            assert!(
                dispatch_res_a.is_ok(),
                "Dispatch failed for host chain a with error: {dispatch_res_a:?}"
            );
            let client_height_a = ctx_a
                .query_client_full_state(&client_on_a_for_b)
                .unwrap()
                .latest_height();
            assert_eq!(client_height_a, ctx_b.query_latest_height().unwrap());
        }
    }
}
