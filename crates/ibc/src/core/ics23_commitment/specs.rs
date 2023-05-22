//! Defines proof specs, which encode the structure of proofs

use crate::prelude::*;
use ics23::{InnerSpec as Ics23InnerSpec, LeafOp as Ics23LeafOp, ProofSpec as Ics23ProofSpec};

/// An array of proof specifications.
///
/// This type encapsulates different types of proof specifications, mostly predefined, e.g., for
/// Cosmos-SDK.
/// Additionally, this type also aids in the conversion from `ProofSpec` types from crate `ics23`
/// into proof specifications as represented in the `ibc_proto` type; see the
/// `From` trait(s) below.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProofSpecs(Vec<ProofSpec>);

impl ProofSpecs {
    /// Returns the specification for Cosmos-SDK proofs
    pub fn cosmos() -> Self {
        vec![
            ics23::iavl_spec(),       // Format of proofs-iavl (iavl merkle proofs)
            ics23::tendermint_spec(), // Format of proofs-tendermint (crypto/ merkle SimpleProof)
        ]
        .into()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Default for ProofSpecs {
    fn default() -> Self {
        Self::cosmos()
    }
}

impl From<Vec<Ics23ProofSpec>> for ProofSpecs {
    fn from(ics23_specs: Vec<Ics23ProofSpec>) -> Self {
        Self(ics23_specs.into_iter().map(ProofSpec).collect())
    }
}

impl From<ProofSpecs> for Vec<Ics23ProofSpec> {
    fn from(specs: ProofSpecs) -> Self {
        specs.0.into_iter().map(|spec| spec.0).collect()
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
struct ProofSpec(Ics23ProofSpec);

impl Eq for ProofSpec {}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
struct LeafOp(Ics23LeafOp);

impl Eq for LeafOp {}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
struct InnerSpec(Ics23InnerSpec);

impl Eq for InnerSpec {}
