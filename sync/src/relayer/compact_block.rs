use ckb_core::header::{Header, HeaderBuilder};
use ckb_core::transaction::{IndexTransaction, ProposalShortId, Transaction};
use ckb_core::uncle::UncleBlock;
use ckb_protocol::{self, cast, FlatbuffersVectorIterator};
use failure::Error as FailureError;
use numext_fixed_hash::H256;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CompactBlock {
    pub header: Header,
    pub uncles: Vec<UncleBlock>,
    pub short_ids: Vec<ProposalShortId>,
    pub prefilled_transactions: Vec<IndexTransaction>,
    pub proposals: Vec<ProposalShortId>,
}

impl Default for CompactBlock {
    fn default() -> Self {
        let header = HeaderBuilder::default().build();
        Self {
            header,
            uncles: Default::default(),
            short_ids: Default::default(),
            prefilled_transactions: Default::default(),
            proposals: Default::default(),
        }
    }
}

impl<'a> TryFrom<ckb_protocol::CompactBlock<'a>> for CompactBlock {
    type Error = FailureError;

    fn try_from(b: ckb_protocol::CompactBlock<'a>) -> Result<Self, Self::Error> {
        let header = cast!(b.header())?;
        let short_ids: Result<Vec<_>, FailureError> = cast!(b.short_ids())?
            .iter()
            .map(TryInto::try_into)
            .collect();
        let prefilled_transactions: Result<Vec<_>, FailureError> =
            FlatbuffersVectorIterator::new(cast!(b.prefilled_transactions())?)
                .map(TryInto::try_into)
                .collect();

        let uncles: Result<Vec<_>, FailureError> =
            FlatbuffersVectorIterator::new(cast!(b.uncles())?)
                .map(TryInto::try_into)
                .collect();
        let proposals: Result<Vec<_>, FailureError> = cast!(b.proposals())?
            .iter()
            .map(TryInto::try_into)
            .collect();

        Ok(CompactBlock {
            header: header.try_into()?,
            short_ids: short_ids?,
            prefilled_transactions: prefilled_transactions?,
            uncles: uncles?,
            proposals: proposals?,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BlockProposal {
    pub transactions: Vec<Transaction>,
}

impl<'a> TryFrom<ckb_protocol::BlockProposal<'a>> for BlockProposal {
    type Error = FailureError;

    fn try_from(b: ckb_protocol::BlockProposal<'a>) -> Result<Self, Self::Error> {
        let transactions: Result<Vec<_>, FailureError> =
            FlatbuffersVectorIterator::new(cast!(b.transactions())?)
                .map(TryInto::try_into)
                .collect();

        Ok(BlockProposal {
            transactions: transactions?,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GetBlockProposal {
    pub block_hash: H256,
    pub proposals: Vec<ProposalShortId>,
}

impl<'a> TryFrom<ckb_protocol::GetBlockProposal<'a>> for GetBlockProposal {
    type Error = FailureError;

    fn try_from(b: ckb_protocol::GetBlockProposal<'a>) -> Result<Self, Self::Error> {
        let block_hash = cast!(b.block_hash())?;
        let proposals: Result<Vec<_>, FailureError> = cast!(b.proposals())?
            .iter()
            .map(TryInto::try_into)
            .collect();

        Ok(GetBlockProposal {
            block_hash: block_hash.try_into()?,
            proposals: proposals?,
        })
    }
}
