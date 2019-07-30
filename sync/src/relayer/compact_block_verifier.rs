use crate::relayer::compact_block::CompactBlock;
use crate::relayer::error::{Error, Misbehavior};
use ckb_core::transaction::ProposalShortId;
use std::collections::HashSet;

pub struct CompactBlockVerifier {
    prefilled: PrefilledVerifier,
    short_ids: ShortIdsVerifier,
}

impl CompactBlockVerifier {
    pub(crate) fn new() -> Self {
        Self {
            prefilled: PrefilledVerifier::new(),
            short_ids: ShortIdsVerifier::new(),
        }
    }

    pub(crate) fn verify(&self, block: &CompactBlock) -> Result<(), Error> {
        self.prefilled.verify(block)?;
        self.short_ids.verify(block)?;
        Ok(())
    }
}

pub struct PrefilledVerifier {}

impl PrefilledVerifier {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn verify(&self, block: &CompactBlock) -> Result<(), Error> {
        let prefilled_transactions = &block.prefilled_transactions;
        let short_ids = &block.short_ids;
        let txs_len = prefilled_transactions.len() + short_ids.len();

        // Check the prefilled_transactions appears to have included the cellbase
        if prefilled_transactions.is_empty() || prefilled_transactions[0].index != 0 {
            return Err(Error::Misbehavior(Misbehavior::CellbaseNotPrefilled));
        }

        // Check indices order of prefilled transactions
        let unordered = prefilled_transactions
            .as_slice()
            .windows(2)
            .any(|pt| pt[0].index >= pt[1].index);
        if unordered {
            return Err(Error::Misbehavior(
                Misbehavior::UnorderedPrefilledTransactions,
            ));
        }

        // Check highest prefilled index is less then length of block transactions
        if !prefilled_transactions.is_empty()
            && prefilled_transactions.last().unwrap().index >= txs_len
        {
            return Err(Error::Misbehavior(
                Misbehavior::OverflowPrefilledTransactions,
            ));
        }

        Ok(())
    }
}

pub struct ShortIdsVerifier {}

impl ShortIdsVerifier {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn verify(&self, block: &CompactBlock) -> Result<(), Error> {
        let prefilled_transactions = &block.prefilled_transactions;
        let short_ids = &block.short_ids;
        let short_ids_set: HashSet<&ProposalShortId> = short_ids.iter().collect();

        // Check duplicated short ids
        if short_ids.len() != short_ids_set.len() {
            return Err(Error::Misbehavior(Misbehavior::DuplicatedShortIds));
        }

        // Check intersection of prefilled transactions and short ids
        let is_intersect = prefilled_transactions
            .iter()
            .any(|pt| short_ids_set.contains(&pt.transaction.proposal_short_id()));
        if is_intersect {
            return Err(Error::Misbehavior(
                Misbehavior::IntersectedPrefilledTransactions,
            ));
        }

        Ok(())
    }
}
