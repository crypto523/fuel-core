use super::model::{BlockHeight, FuelBlock, SealedFuelBlock};
use crate::model::ConsensusVote;
use fuel_tx::Transaction;
use std::sync::Arc;
use tokio::sync::oneshot;

pub enum TransactionBroadcast {
    NewTransaction(Transaction),
}

pub enum ConsensusBroadcast {
    NewVote(ConsensusVote),
}

pub enum BlockBroadcast {
    /// fuel block without consensus data
    NewBlock(FuelBlock),
}

pub enum P2pRequestEvent {
    RequestBlock {
        height: BlockHeight,
        response: oneshot::Sender<SealedFuelBlock>,
    },
    BroadcastNewTransaction {
        transaction: Arc<Transaction>,
    },
    BroadcastNewBlock {
        block: Arc<FuelBlock>,
    },
    BroadcastConsensusVote {
        vote: Arc<ConsensusVote>,
    },
}
