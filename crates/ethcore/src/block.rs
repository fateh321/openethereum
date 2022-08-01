// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// This file is part of OpenEthereum.

// OpenEthereum is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// OpenEthereum is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with OpenEthereum.  If not, see <http://www.gnu.org/licenses/>.

//! Base data structure of this module is `Block`.
//!
//! Blocks can be produced by a local node or they may be received from the network.
//!
//! To create a block locally, we start with an `OpenBlock`. This block is mutable
//! and can be appended to with transactions and uncles.
//!
//! When ready, `OpenBlock` can be closed and turned into a `ClosedBlock`. A `ClosedBlock` can
//! be reopend again by a miner under certain circumstances. On block close, state commit is
//! performed.
//!
//! `LockedBlock` is a version of a `ClosedBlock` that cannot be reopened. It can be sealed
//! using an engine.
//!
//! `ExecutedBlock` is an underlaying data structure used by all structs above to store block
//! related info.

use std::{cmp, collections::HashSet, ops, sync::Arc};
use std::collections::HashMap;

use bytes::Bytes;
use ethereum_types::{Address, Bloom, H256, U256};

use engines::EthEngine;
use error::{BlockError, Error};
use factory::Factories;
use state::State;
use state_db::StateDB;
use trace::Tracing;
use triehash::ordered_trie_root;
use unexpected::{Mismatch, OutOfBounds};
use verification::PreverifiedBlock;
use vm::{EnvInfo, LastHashes};

use hash::keccak;
use rlp::{encode_list, RlpStream};
use hyperproofs::AggProof;
use stats::prometheus::register_int_counter;
use types::{
    header::{ExtendedHeader, Header},
    receipt::{TransactionOutcome, TypedReceipt},
    transaction::{Error as TransactionError, SignedTransaction},
};

/// Block that is ready for transactions to be added.
///
/// It's a bit like a Vec<Transaction>, except that whenever a transaction is pushed, we execute it and
/// maintain the system `state()`. We also archive execution receipts in preparation for later block creation.
pub struct OpenBlock<'x> {
    block: ExecutedBlock,
    engine: &'x dyn EthEngine,
}

/// Just like `OpenBlock`, except that we've applied `Engine::on_close_block`, finished up the non-seal header fields,
/// and collected the uncles.
///
/// There is no function available to push a transaction.
#[derive(Clone)]
pub struct ClosedBlock {
    block: ExecutedBlock,
    unclosed_state: State<StateDB>,
}

/// Just like `ClosedBlock` except that we can't reopen it and it's faster.
///
/// We actually store the post-`Engine::on_close_block` state, unlike in `ClosedBlock` where it's the pre.
#[derive(Clone)]
pub struct LockedBlock {
    block: ExecutedBlock,
}

/// A block that has a valid seal.
///
/// The block's header has valid seal arguments. The block cannot be reversed into a `ClosedBlock` or `OpenBlock`.
pub struct SealedBlock {
    block: ExecutedBlock,
}

/// An internal type for a block's common elements.
#[derive(Clone)]
pub struct ExecutedBlock {
    /// Executed block header.
    pub header: Header,
    /// Executed transactions.
    pub transactions: Vec<SignedTransaction>,
    /// Uncles.
    pub uncles: Vec<Header>,
    /// Transaction receipts.
    pub receipts: Vec<TypedReceipt>,
    /// Hashes of already executed transactions.
    pub transactions_set: HashSet<H256>,
    /// Underlaying state.
    pub state: State<StateDB>,
    /// Transaction traces.
    pub traces: Tracing,
    /// Hashes of last 256 blocks.
    pub last_hashes: Arc<LastHashes>,
}

impl ExecutedBlock {
    /// Create a new block from the given `state`.
    fn new(state: State<StateDB>, last_hashes: Arc<LastHashes>, tracing: bool) -> ExecutedBlock {
        ExecutedBlock {
            header: Default::default(),
            transactions: Default::default(),
            uncles: Default::default(),
            receipts: Default::default(),
            transactions_set: Default::default(),
            state: state,
            traces: if tracing {
                Tracing::enabled()
            } else {
                Tracing::Disabled
            },
            last_hashes: last_hashes,
        }
    }

    /// Get the environment info concerning this block.
    pub fn env_info(&self) -> EnvInfo {
        // TODO: memoise.
        EnvInfo {
            number: self.header.number(),
            author: self.header.author().clone(),
            timestamp: self.header.timestamp(),
            difficulty: self.header.difficulty().clone(),
            last_hashes: self.last_hashes.clone(),
            gas_used: self.receipts.last().map_or(U256::zero(), |r| r.gas_used),
            gas_limit: *self.header.gas_limit(),
            base_fee: self.header.base_fee(),
        }
    }

    /// Get mutable access to a state.
    pub fn state_mut(&mut self) -> &mut State<StateDB> {
        &mut self.state
    }

    /// Get mutable reference to traces.
    pub fn traces_mut(&mut self) -> &mut Tracing {
        &mut self.traces
    }
}

/// Trait for an object that owns an `ExecutedBlock`
pub trait Drain {
    /// Returns `ExecutedBlock`
    fn drain(self) -> ExecutedBlock;
}

impl<'x> OpenBlock<'x> {
    pub fn new_shard<'a, I: IntoIterator<Item = ExtendedHeader>>(
        engine: &'x dyn EthEngine,
        factories: Factories,
        tracing: bool,
        db: StateDB,
        parent: &Header,
        last_hashes: Arc<LastHashes>,
        author: Address,
        gas_range_target: (U256, U256),
        extra_data: Bytes,
        is_epoch_begin: bool,
        ancestry: I,
        state_root:H256,
    ) -> Result<Self, Error> {
        let number = parent.number() + 1;

        // t_nb 8.1.1 get parent StateDB.
        //this part is different
        // #[cfg(feature = "shard")]
        let state = State::from_existing(
            db,
            state_root.clone(),
            engine.account_start_nonce(number),
            factories,
        )?;
        let mut r = OpenBlock {
            block: ExecutedBlock::new(state, last_hashes, tracing),
            engine: engine,
        };

        r.block.header.set_parent_hash(parent.hash());
        r.block.header.set_number(number);
        r.block.header.set_author(author);
        r.block
            .header
            .set_timestamp(engine.open_block_header_timestamp(parent.timestamp()));
        r.block.header.set_extra_data(extra_data);
        r.block
            .header
            .set_base_fee(engine.calculate_base_fee(parent));

        let gas_floor_target = cmp::max(gas_range_target.0, engine.params().min_gas_limit);
        let gas_ceil_target = cmp::max(gas_range_target.1, gas_floor_target);

        // t_nb 8.1.2 It calculated child gas limits should be.
        engine.machine().populate_from_parent(
            &mut r.block.header,
            parent,
            gas_floor_target,
            gas_ceil_target,
        );
        // t_nb 8.1.3 this adds engine specific things
        engine.populate_from_parent(&mut r.block.header, parent);

        // t_nb 8.1.3 updating last hashes and the DAO fork, for ethash.
        engine.machine().on_new_block(&mut r.block)?;
        engine.on_new_block(&mut r.block, is_epoch_begin, &mut ancestry.into_iter())?;

        Ok(r)
    }
    /// t_nb 8.1 Create a new `OpenBlock` ready for transaction pushing.
    pub fn new<'a, I: IntoIterator<Item = ExtendedHeader>>(
        engine: &'x dyn EthEngine,
        factories: Factories,
        tracing: bool,
        db: StateDB,
        parent: &Header,
        last_hashes: Arc<LastHashes>,
        author: Address,
        gas_range_target: (U256, U256),
        extra_data: Bytes,
        is_epoch_begin: bool,
        ancestry: I,
    ) -> Result<Self, Error> {
        let number = parent.number() + 1;

        // t_nb 8.1.1 get parent StateDB.
        let state = State::from_existing(
            db,
            parent.state_root().clone(),
            engine.account_start_nonce(number),
            factories,
        )?;
        let mut r = OpenBlock {
            block: ExecutedBlock::new(state, last_hashes, tracing),
            engine: engine,
        };

        r.block.header.set_parent_hash(parent.hash());
        r.block.header.set_number(number);
        r.block.header.set_author(author);
        r.block
            .header
            .set_timestamp(engine.open_block_header_timestamp(parent.timestamp()));
        r.block.header.set_extra_data(extra_data);
        r.block
            .header
            .set_base_fee(engine.calculate_base_fee(parent));

        let gas_floor_target = cmp::max(gas_range_target.0, engine.params().min_gas_limit);
        let gas_ceil_target = cmp::max(gas_range_target.1, gas_floor_target);

        // t_nb 8.1.2 It calculated child gas limits should be.
        engine.machine().populate_from_parent(
            &mut r.block.header,
            parent,
            gas_floor_target,
            gas_ceil_target,
        );
        // t_nb 8.1.3 this adds engine specific things
        engine.populate_from_parent(&mut r.block.header, parent);

        // t_nb 8.1.3 updating last hashes and the DAO fork, for ethash.
        engine.machine().on_new_block(&mut r.block)?;
        engine.on_new_block(&mut r.block, is_epoch_begin, &mut ancestry.into_iter())?;

        Ok(r)
    }

    pub fn set_mined_status(&mut self, status: Option<bool>){
        self.block.state.set_mined_status(status);
    }
    pub fn set_incomplete_txn(&mut self, txn: Vec<SignedTransaction>){
        self.block.state.set_incomplete_txn(txn);
    }
    /// Alter the timestamp of the block.
    pub fn set_timestamp(&mut self, timestamp: u64) {
        self.block.header.set_timestamp(timestamp);
    }

    /// Removes block gas limit.
    pub fn remove_gas_limit(&mut self) {
        self.block.header.set_gas_limit(U256::max_value());
    }

    /// Set block gas limit.
    pub fn set_gas_limit(&mut self, gas_limit: U256) {
        self.block.header.set_gas_limit(gas_limit);
    }

    // t_nb 8.4 Add an uncle to the block, if possible.
    ///
    /// NOTE Will check chain constraints and the uncle number but will NOT check
    /// that the header itself is actually valid.
    pub fn push_uncle(&mut self, valid_uncle_header: Header) -> Result<(), BlockError> {
        let max_uncles = self.engine.maximum_uncle_count(self.block.header.number());
        if self.block.uncles.len() + 1 > max_uncles {
            return Err(BlockError::TooManyUncles(OutOfBounds {
                min: None,
                max: Some(max_uncles),
                found: self.block.uncles.len() + 1,
            }));
        }
        // TODO: check number
        // TODO: check not a direct ancestor (use last_hashes for that)
        self.block.uncles.push(valid_uncle_header);
        Ok(())
    }

    /// Push a transaction into the block.
    ///
    /// If valid, it will be executed, and archived together with the receipt.
    pub fn push_transaction(
        &mut self,
        t: SignedTransaction,
        h: Option<H256>,
    ) -> Result<&TypedReceipt, Error> {
        if self.block.transactions_set.contains(&t.hash()) {
            return Err(TransactionError::AlreadyImported.into());
        }
        // let mut wtr = csv::Writer::from_writer();
        // #[cfg(feature = "shard")]
        //here we will verify the proof if any
        let data = t.shard_proof_data();
        if !data.is_empty(){
            println!("data looks like{:?}", data);
            AggProof::resetAddressBalanceVerify(t.shard_id());
            for datum in data{
                println!("address looks like{} balance looks like {} shard looks like {}",datum.0.to_low_u64_be().rem_euclid(2u64.pow(16)),datum.1, t.shard_id());
                AggProof::pushAddressBalanceVerify(datum.0.to_low_u64_be().rem_euclid(2u64.pow(16)),datum.1.to_string(),t.shard_id());
            }
            let proof_result = AggProof::verifyProof(t.shard_proof(),t.shard_id(),0u64);
            println!("verification is {:?}", proof_result);

        }
        let env_info = self.block.env_info();
        // #[cfg(feature = "shard")]
        let sender = t.original_sender();
        // debug!(target: "miner", "transaction looks like {:?}", t);
        let mut t= if !t.contains_balance(){
            AggProof::incr_bal_read_count(1u64);
            let mut balance = self.state.balance(&sender)?;
            let mut begin_round_balance = self.state.hash_map_beginning_storage_at(&sender);
            if begin_round_balance.1 {
                balance = begin_round_balance.0;
            }
            t.with_balance(balance)
        } else{
            t
        };
        // #[cfg(feature = "shard")]
        //clear address txn vec and add original sender's address only if transaction is complete.
        self.block.state.clear_address_txn_vec();
        if !t.is_incomplete() && t.is_shard(){
            self.block.state.push_address_txn_vec(sender);
        }
        // we turn revert flag down before.
        self.block.state.reverted(false);
        self.block.state.clear_hash_map_cache();
        //set is_create_txn flag
        if !t.is_shard(){
            self.block.state.set_is_create_txn(true);
        }else {
            self.block.state.set_is_create_txn(false);
        }
        let mut outcome = self.block.state.default_apply_result().unwrap();
        self.block.state.clear_temp_sstore_val();
        self.block.state.clear_temp_sstore_delta();
        match self.block.state.get_mined_status(){
            Some(true) => {
                if !t.tx().data.is_empty(){
                    if !t.is_incomplete(){
                        if !t.is_shard(){ //mined, smart-contract, complete, legacy = CREATE
                            debug!(target: "txn", "legacy state.apply() from miner");
                            self.block.state.clear_data_hashmap_txn();
                            self.block.state.set_next_shard(999u64);
                            self.block.state.set_txn_status(Some(true));
                            outcome = self.block.state.apply(
                                &env_info,
                                self.engine.machine(),
                                &t,
                                self.block.traces.is_enabled(),
                            )?;
                        }else {
                            //mined, smart-contract, complete, shard = new txn
                            debug!(target: "txn", "complete txn, shard state.apply() from miner with None flag");
                            self.block.state.clear_data_hashmap_txn();
                            self.block.state.clear_hash_map_cache();
                            self.block.state.set_next_shard(999u64);
                            for (key, val) in t.shard_data_hashmap().iter() {
                                self.block.state.hash_map_txn_insert(key.clone(), val.clone())
                            }
                            self.block.state.set_txn_status(None);
                            outcome = self.block.state.fake_apply(
                                &env_info,
                                self.engine.machine(),
                                &t,
                                self.block.traces.is_enabled(),
                            )?;
                            if self.block.state.txn_complete_status() == None {
                                debug!(target: "txn", "complete txn succesfully executed");
                                //clear cache
                                // self.block.state.clear_hash_map_cache();
                                // self.block.state.set_txn_status(Some(true));
                                // outcome = self.block.state.apply(
                                //     &env_info,
                                //     self.engine.machine(),
                                //     &t,
                                //     self.block.traces.is_enabled(),
                                // )?;
                                t.hash_map_replace_with(self.block.state.data_hashmap_txn());
                            } else {
                                debug!(target: "txn", "complete txn from miner set to incomplete");
                                t.hash_map_replace_with(self.block.state.data_hashmap_txn());
                                t.set_next_shard(self.block.state.get_next_shard());
                                t.set_incomplete(1u64);
                            }

                        }
                    } else{ // mined, smart-contract, incomplete
                    debug!(target: "txn", "incomplete txn from miner state.apply() with None flag");
                        self.block.state.clear_data_hashmap_txn();
                        self.block.state.clear_hash_map_cache();
                        self.block.state.set_next_shard(999u64);
                        for (key, val) in t.shard_data_hashmap().iter() {
                            self.block.state.hash_map_txn_insert(key.clone(), val.clone())
                        }
                        self.block.state.set_txn_status(None);
                        outcome = self.block.state.fake_apply(
                            &env_info,
                            self.engine.machine(),
                            &t,
                            self.block.traces.is_enabled(),
                        )?;
                        if self.block.state.txn_complete_status() == None {
                            debug!(target: "txn", "incomplete txn from miner successfully executed");
                            // self.block.state.set_txn_status(Some(true));
                            //clear cache
                            // self.block.state.clear_hash_map_cache();
                            // outcome = self.block.state.apply(
                            //     &env_info,
                            //     self.engine.machine(),
                            //     &t,
                            //     self.block.traces.is_enabled(),
                            // )?;
                            t.set_incomplete(0u64);
                            t.hash_map_replace_with(self.block.state.data_hashmap_txn());
                        } else {
                            debug!(target: "txn", "incomplete txn from miner set to incomplete again");
                            t.hash_map_replace_with(self.block.state.data_hashmap_txn());
                            t.set_next_shard(self.block.state.get_next_shard());
                            t.set_incomplete(1u64);
                        }
                    }
                } else{ //mined, CALL transfer
                debug!(target: "txn", "CALL transfer txn from miner");
                    self.block.state.clear_data_hashmap_txn();
                    self.block.state.set_next_shard(999u64);
                    for (key, val) in t.shard_data_hashmap().iter() {
                        self.block.state.hash_map_txn_insert(key.clone(), val.clone())
                    }
                    self.block.state.set_txn_status(Some(true));
                    outcome = self.block.state.apply(
                        &env_info,
                        self.engine.machine(),
                        &t,
                        self.block.traces.is_enabled(),
                    )?;
                }
            }
            _ => {if t.is_incomplete(){//enact, incomplete
                //do nothing in terms of state.apply
            debug!(target: "txn", "incomplete txn in enact, do nothing");
                self.block.state.inc_nonce(&t.sender())?;
                if t.get_next_shard() == AggProof::get_shard(){
                    self.block.state.push_incomplete_txn(t.clone());
                }
            } else{ //enact, complete
            if !t.tx().data.is_empty() && t.is_shard(){
                // complete smart contract shard txn
                debug!(target: "txn", "complete smart contract txn in enact, just state.fake_apply()");
                self.block.state.clear_data_hashmap_txn();
                self.block.state.set_next_shard(999u64);
                for (key, val) in t.shard_data_hashmap().iter() {
                    self.block.state.hash_map_txn_insert(key.clone(), val.clone())
                }
                // self.block.state.set_txn_status(Some(true));
                self.block.state.set_txn_status(None);
                // outcome = self.block.state.apply(
                outcome = self.block.state.fake_apply(
                    &env_info,
                    self.engine.machine(),
                    &t,
                    self.block.traces.is_enabled(),
                )?;
                assert_eq!(self.block.state.txn_complete_status(), None);
            }else {
                // complete legacy or call txn
                debug!(target: "txn", "complete call or legacy txn in enact, just state.apply()");
                self.block.state.clear_data_hashmap_txn();
                self.block.state.set_next_shard(999u64);
                for (key, val) in t.shard_data_hashmap().iter() {
                    self.block.state.hash_map_txn_insert(key.clone(), val.clone())
                }
                self.block.state.set_txn_status(Some(true));
                // self.block.state.set_txn_status(None);
                outcome = self.block.state.apply(
                // outcome = self.block.state.fake_apply(
                    &env_info,
                    self.engine.machine(),
                    &t,
                    self.block.traces.is_enabled(),
                )?;
                // assert_eq!(self.block.state.txn_complete_status(), None);
            }

            }

            }
        }



        // if t.is_incomplete() && self.block.state.get_mined_status()==Some(false){
        //     // do nothing
        // } else {
        //     self.block.state.clear_data_hashmap_txn();
        //     self.block.state.set_next_shard(999u64);
        //     for (key, val) in t.shard_data_hashmap().iter() {
        //         self.block.state.hash_map_txn_insert(key.clone(), val.clone())
        //     }
        //     if self.block.state.get_mined_status()==Some(true){
        //         self.block.state.set_txn_status(None);
        //     } else {
        //         self.block.state.set_txn_status(Some(true));
        //     }
        //     if !t.is_shard(){
        //         //set true for legacy CREATE transactions
        //         self.block.state.set_txn_status(Some(true));
        //     }
        //     outcome = self.block.state.apply(
        //         &env_info,
        //         self.engine.machine(),
        //         &t,
        //         self.block.traces.is_enabled(),
        //     )?;
        //     if !t.tx().data.is_empty() && self.block.state.txn_complete_status() == None && t.is_shard() && self.block.state.get_mined_status()==Some(true){
        //         self.block.state.set_txn_status(Some(true));
        //         outcome = self.block.state.apply(
        //             &env_info,
        //             self.engine.machine(),
        //             &t,
        //             self.block.traces.is_enabled(),
        //         )?;
        //         t.hash_map_replace_with(self.block.state.data_hashmap_txn());
        //     } else {
        //         if !t.tx().data.is_empty() && t.is_shard() && self.block.state.txn_complete_status() == Some(false){
        //             t.hash_map_replace_with(self.block.state.data_hashmap_txn());
        //             t.set_next_shard(self.block.state.get_next_shard());
        //             t.set_incomplete(1u64);
        //         }else {
        //             //do nothing
        //         }
        //     }
        // }
        if !t.is_incomplete(){
            AggProof::incr_hop_count(t.get_hop_count()+1);
            if self.block.state.is_reverted(){
                AggProof::incr_reverted_count();
            }
        }
        self.block
            .transactions_set
            .insert(h.unwrap_or_else(|| t.hash()));
        debug!(target: "txn", "push_txn, pushing t in the block with hashmap {:?}", t.shard_data_hashmap());
        self.block.transactions.push(t.into());
        if let Tracing::Enabled(ref mut traces) = self.block.traces {
            traces.push(outcome.trace.into());
        }
        self.block.receipts.push(outcome.receipt);
        Ok(self
            .block
            .receipts
            .last()
            .expect("receipt just pushed; qed"))
    }

    /// Push transactions onto the block.
    #[cfg(not(feature = "slow-blocks"))]
    fn push_transactions(&mut self, transactions: Vec<SignedTransaction>) -> Result<(), Error> {
        for t in transactions {
            self.push_transaction(t, None)?;
        }
        Ok(())
    }

    /// Push transactions onto the block.
    #[cfg(feature = "slow-blocks")]
    fn push_transactions(&mut self, transactions: Vec<SignedTransaction>) -> Result<(), Error> {
        use std::time;

        let slow_tx = option_env!("SLOW_TX_DURATION")
            .and_then(|v| v.parse().ok())
            .unwrap_or(100);
        for t in transactions {
            let hash = t.hash();
            let start = time::Instant::now();
            self.push_transaction(t, None)?;
            let took = start.elapsed();
            let took_ms = took.as_secs() * 1000 + took.subsec_nanos() as u64 / 1000000;
            if took > time::Duration::from_millis(slow_tx) {
                warn!(
                    "Heavy ({} ms) transaction in block {:?}: {:?}",
                    took_ms,
                    self.block.header.number(),
                    hash
                );
            }
            debug!(target: "tx", "Transaction {:?} took: {} ms", hash, took_ms);
        }

        Ok(())
    }

    /// Populate self from a header.
    fn populate_from(&mut self, header: &Header) {
        self.block.header.set_difficulty(*header.difficulty());
        self.block.header.set_gas_limit(*header.gas_limit());
        self.block.header.set_timestamp(header.timestamp());
        self.block.header.set_uncles_hash(*header.uncles_hash());
        self.block
            .header
            .set_transactions_root(*header.transactions_root());
        // TODO: that's horrible. set only for backwards compatibility
        if header.extra_data().len() > self.engine.maximum_extra_data_size() {
            warn!("Couldn't set extradata. Ignoring.");
        } else {
            self.block
                .header
                .set_extra_data(header.extra_data().clone());
        }
    }

    /// Turn this into a `ClosedBlock`.
    pub fn close(self) -> Result<ClosedBlock, Error> {
        let unclosed_state = self.block.state.clone();
        let locked = self.close_and_lock()?;

        Ok(ClosedBlock {
            block: locked.block,
            unclosed_state,
        })
    }

    /// t_nb 8.5 Turn this into a `LockedBlock`.
    pub fn close_and_lock(self) -> Result<LockedBlock, Error> {
        let mut s = self;

        // t_nb 8.5.1 engine applies block rewards (Ethash and AuRa do.Clique is empty)
        s.engine.on_close_block(&mut s.block)?;

        // t_nb 8.5.2 commit account changes from cache to tree
        s.block.state.commit()?;

        // t_nb 8.5.3 fill open block header with all other fields
        s.block.header.set_transactions_root(ordered_trie_root(
            s.block.transactions.iter().map(|e| e.encode()),
        ));
        let uncle_bytes = encode_list(&s.block.uncles);
        s.block.header.set_uncles_hash(keccak(&uncle_bytes));
        s.block.header.set_state_root(s.block.state.root().clone());
        // #[cfg(feature = "shard")]
        // we set state root to default value for all nodes
        // s.block.header.set_state_root(H256::default());
        debug!(target: "block", "Adding block state root {:?}",s.block.state.root().clone());
        s.block.header.set_receipts_root(ordered_trie_root(
            s.block.receipts.iter().map(|r| r.encode()),
        ));
        s.block
            .header
            .set_log_bloom(s.block.receipts.iter().fold(Bloom::zero(), |mut b, r| {
                b.accrue_bloom(&r.log_bloom);
                b
            }));
        s.block.header.set_gas_used(
            s.block
                .receipts
                .last()
                .map_or_else(U256::zero, |r| r.gas_used),
        );

        Ok(LockedBlock { block: s.block })
    }
    pub fn set_hash_map_global(&mut self, h: Vec<HashMap<Address,U256>>){
        self.block.state.set_hash_map_global(h);
    }

    pub fn set_hash_map_round_beginning(&mut self, h: HashMap<Address,U256>){
        self.block.state.set_hash_map_round_beginning(h);
    }
    pub fn set_incr_bal_round(&mut self, h: HashMap<Address,U256>){
        self.block.state.set_incr_bal_round(h);
    }
    #[cfg(test)]
    /// Return mutable block reference. To be used in tests only.
    pub fn block_mut(&mut self) -> &mut ExecutedBlock {
        &mut self.block
    }
}

impl<'a> ops::Deref for OpenBlock<'a> {
    type Target = ExecutedBlock;

    fn deref(&self) -> &Self::Target {
        &self.block
    }
}

impl ops::Deref for ClosedBlock {
    type Target = ExecutedBlock;

    fn deref(&self) -> &Self::Target {
        &self.block
    }
}

impl ops::Deref for LockedBlock {
    type Target = ExecutedBlock;

    fn deref(&self) -> &Self::Target {
        &self.block
    }
}

impl ops::Deref for SealedBlock {
    type Target = ExecutedBlock;

    fn deref(&self) -> &Self::Target {
        &self.block
    }
}

impl ClosedBlock {
    /// Turn this into a `LockedBlock`, unable to be reopened again.
    pub fn lock(self) -> LockedBlock {
        LockedBlock { block: self.block }
    }

    /// Given an engine reference, reopen the `ClosedBlock` into an `OpenBlock`.
    pub fn reopen(self, engine: &dyn EthEngine) -> OpenBlock {
        // revert rewards (i.e. set state back at last transaction's state).
        let mut block = self.block;
        block.state = self.unclosed_state;
        OpenBlock {
            block: block,
            engine: engine,
        }
    }
}

impl LockedBlock {
    /// Removes outcomes from receipts and updates the receipt root.
    ///
    /// This is done after the block is enacted for historical reasons.
    /// We allow inconsistency in receipts for some chains if `validate_receipts_transition`
    /// is set to non-zero value, so the check only happens if we detect
    /// unmatching root first and then fall back to striped receipts.
    pub fn strip_receipts_outcomes(&mut self) {
        for receipt in &mut self.block.receipts {
            receipt.outcome = TransactionOutcome::Unknown;
        }
        self.block.header.set_receipts_root(ordered_trie_root(
            self.block.receipts.iter().map(|r| r.encode()),
        ));
    }

    /// Provide a valid seal in order to turn this into a `SealedBlock`.
    ///
    /// NOTE: This does not check the validity of `seal` with the engine.
    pub fn seal(self, engine: &dyn EthEngine, seal: Vec<Bytes>) -> Result<SealedBlock, Error> {
        let expected_seal_fields = engine.seal_fields(&self.header);
        let mut s = self;
        if seal.len() != expected_seal_fields {
            Err(BlockError::InvalidSealArity(Mismatch {
                expected: expected_seal_fields,
                found: seal.len(),
            }))?;
        }

        s.block.header.set_seal(seal);
        engine.on_seal_block(&mut s.block)?;
        s.block.header.compute_hash();

        Ok(SealedBlock { block: s.block })
    }

    /// Provide a valid seal in order to turn this into a `SealedBlock`.
    /// This does check the validity of `seal` with the engine.
    /// Returns the `ClosedBlock` back again if the seal is no good.
    /// TODO(https://github.com/openethereum/openethereum/issues/10407): This is currently only used in POW chain call paths, we should really merge it with seal() above.
    pub fn try_seal(self, engine: &dyn EthEngine, seal: Vec<Bytes>) -> Result<SealedBlock, Error> {
        let mut s = self;
        s.block.header.set_seal(seal);
        s.block.header.compute_hash();

        // TODO: passing state context to avoid engines owning it?
        engine.verify_local_seal(&s.block.header)?;
        Ok(SealedBlock { block: s.block })
    }
}

impl Drain for LockedBlock {
    fn drain(self) -> ExecutedBlock {
        self.block
    }
}

impl SealedBlock {
    /// Get the RLP-encoding of the block.
    pub fn rlp_bytes(&self) -> Bytes {
        let mut block_rlp = RlpStream::new_list(3);
        block_rlp.append(&self.block.header);
        SignedTransaction::rlp_append_list(&mut block_rlp, &self.block.transactions);
        block_rlp.append_list(&self.block.uncles);
        block_rlp.out()
    }
}

impl Drain for SealedBlock {
    fn drain(self) -> ExecutedBlock {
        self.block
    }
}

// t_nb 8.0 Enact the block given by block header, transactions and uncles
pub(crate) fn enact(
    header: Header,
    transactions: Vec<SignedTransaction>,
    uncles: Vec<Header>,
    engine: &dyn EthEngine,
    tracing: bool,
    db: StateDB,
    parent: &Header,
    last_hashes: Arc<LastHashes>,
    factories: Factories,
    hash_map_global: Vec<HashMap<Address, U256>>,
    hash_map_round_beginning: HashMap<Address,U256>,
    incr_bal_round: HashMap<Address,U256>,
    state_root: H256,
    is_epoch_begin: bool,
    ancestry: &mut dyn Iterator<Item = ExtendedHeader>,
) -> Result<LockedBlock, Error> {
    // For trace log
    debug!(target: "txn", "^^^^^^^^^^^^entering trace_state 2^^^^^^^^^^");
    let trace_state = if log_enabled!(target: "enact", ::log::Level::Trace) {

        Some(State::from_existing(
            db.boxed_clone(),
            parent.state_root().clone(),
            engine.account_start_nonce(parent.number() + 1),
            factories.clone(),
        )?)
    } else {
        debug!(target: "txn", "^^^^^^^^^^^^entering trace_state 3^^^^^^^^^^");
        None
    };

    // t_nb 8.1 Created new OpenBlock
    let mut b = OpenBlock::new_shard(
        engine,
        factories,
        tracing,
        db,
        parent,
        last_hashes,
        // Engine such as Clique will calculate author from extra_data.
        // this is only important for executing contracts as the 'executive_author'.
        engine.executive_author(&header)?,
        (3141562.into(), 31415620.into()),
        vec![],
        is_epoch_begin,
        ancestry,
        state_root,
    )?;

    if let Some(ref s) = trace_state {
        let author_balance = s.balance(&b.header.author())?;
        trace!(target: "enact", "num={}, root={}, author={}, author_balance={}\n",
				b.block.header.number(), s.root(), b.header.author(), author_balance);
    }
    // #[cfg(feature = "shard")]
    let block_number = b.block.header.number().clone();
    if block_number.rem_euclid(AggProof::shard_count()) ==0{
        trace!(target:"enact", "block number is {}", block_number);
        if block_number != AggProof::get_last_commit_round(){
            AggProof::commit(AggProof::get_shard(),0u64);
            AggProof::set_last_commit_shard(block_number);
        }
    }

    //set mined status to false in the state
    // #[cfg(feature = "shard")]
    b.block.state.set_mined_status(Some(false));
    // set the hash_maps (global and beginning round)
    b.block.state.set_hash_map_global(hash_map_global);
    b.block.state.set_hash_map_round_beginning(hash_map_round_beginning);
    b.block.state.set_incr_bal_round(incr_bal_round);
    // t_nb 8.2 transfer all field from current header to OpenBlock header that we created
    b.populate_from(&header);
    // t_nb 8.3 execute transactions one by one
    b.push_transactions(transactions)?;

    // t_nb 8.4 Push uncles to OpenBlock and check if we have more then max uncles
    for u in uncles {
        b.push_uncle(u)?;
    }

    // t_nb 8.5 close block
    b.close_and_lock()
}

/// t_nb 8.0 Enact the block given by `block_bytes` using `engine` on the database `db` with given `parent` block header
pub fn enact_verified(
    block: PreverifiedBlock,
    engine: &dyn EthEngine,
    tracing: bool,
    db: StateDB,
    parent: &Header,
    last_hashes: Arc<LastHashes>,
    factories: Factories,
    // #[cfg(feature = "shard")]
    hash_map_global: Vec<HashMap<Address,U256>>,
    hash_map_round_beginning: HashMap<Address, U256>,
    incr_bal_round: HashMap<Address,U256>,
    state_root : H256,
    is_epoch_begin: bool,
    ancestry: &mut dyn Iterator<Item = ExtendedHeader>,
) -> Result<LockedBlock, Error> {
    enact(
        block.header,
        block.transactions,
        block.uncles,
        engine,
        tracing,
        db,
        parent,
        last_hashes,
        factories,
        hash_map_global,
        hash_map_round_beginning,
        incr_bal_round,
        state_root,
        is_epoch_begin,
        ancestry,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use engines::EthEngine;
    use error::Error;
    use ethereum_types::Address;
    use factory::Factories;
    use state_db::StateDB;
    use std::sync::Arc;
    use test_helpers::get_temp_state_db;
    use types::{header::Header, transaction::SignedTransaction, view, views::BlockView};
    use verification::queue::kind::blocks::Unverified;
    use vm::LastHashes;

    /// Enact the block given by `block_bytes` using `engine` on the database `db` with given `parent` block header
    fn enact_bytes(
        block_bytes: Vec<u8>,
        engine: &dyn EthEngine,
        tracing: bool,
        db: StateDB,
        parent: &Header,
        last_hashes: Arc<LastHashes>,
        factories: Factories,
    ) -> Result<LockedBlock, Error> {
        let block = Unverified::from_rlp(block_bytes, engine.params().eip1559_transition)?;
        let header = block.header;
        let transactions: Result<Vec<_>, Error> = block
            .transactions
            .into_iter()
            .map(SignedTransaction::new)
            .map(|r| r.map_err(Into::into))
            .collect();
        let transactions = transactions?;

        {
            if ::log::max_level() >= ::log::Level::Trace {
                let s = State::from_existing(
                    db.boxed_clone(),
                    parent.state_root().clone(),
                    engine.account_start_nonce(parent.number() + 1),
                    factories.clone(),
                )?;
                trace!(target: "enact", "num={}, root={}, author={}, author_balance={}\n",
					header.number(), s.root(), header.author(), s.balance(&header.author())?);
            }
        }

        let mut b = OpenBlock::new(
            engine,
            factories,
            tracing,
            db,
            parent,
            last_hashes,
            Address::default(),
            (3141562.into(), 31415620.into()),
            vec![],
            false,
            None,
        )?;

        b.populate_from(&header);
        b.state.set_mined_status(Some(false));
        b.push_transactions(transactions)?;

        for u in block.uncles {
            b.push_uncle(u)?;
        }

        b.close_and_lock()
    }

    /// Enact the block given by `block_bytes` using `engine` on the database `db` with given `parent` block header. Seal the block aferwards
    fn enact_and_seal(
        block_bytes: Vec<u8>,
        engine: &dyn EthEngine,
        tracing: bool,
        db: StateDB,
        parent: &Header,
        last_hashes: Arc<LastHashes>,
        factories: Factories,
    ) -> Result<SealedBlock, Error> {
        let header =
            Unverified::from_rlp(block_bytes.clone(), engine.params().eip1559_transition)?.header;
        Ok(enact_bytes(
            block_bytes,
            engine,
            tracing,
            db,
            parent,
            last_hashes,
            factories,
        )?
        .seal(engine, header.seal().to_vec())?)
    }

    #[test]
    fn open_block() {
        use spec::*;
        let spec = Spec::new_test();
        let genesis_header = spec.genesis_header();
        let db = spec
            .ensure_db_good(get_temp_state_db(), &Default::default())
            .unwrap();
        let last_hashes = Arc::new(vec![genesis_header.hash()]);
        let b = OpenBlock::new(
            &*spec.engine,
            Default::default(),
            false,
            db,
            &genesis_header,
            last_hashes,
            Address::zero(),
            (3141562.into(), 31415620.into()),
            vec![],
            false,
            None,
        )
        .unwrap();
        let b = b.close_and_lock().unwrap();
        let _ = b.seal(&*spec.engine, vec![]);
    }

    #[test]
    fn enact_block() {
        use spec::*;
        let spec = Spec::new_test();
        let engine = &*spec.engine;
        let genesis_header = spec.genesis_header();

        let db = spec
            .ensure_db_good(get_temp_state_db(), &Default::default())
            .unwrap();
        let last_hashes = Arc::new(vec![genesis_header.hash()]);
        let b = OpenBlock::new(
            engine,
            Default::default(),
            false,
            db,
            &genesis_header,
            last_hashes.clone(),
            Address::zero(),
            (3141562.into(), 31415620.into()),
            vec![],
            false,
            None,
        )
        .unwrap()
        .close_and_lock()
        .unwrap()
        .seal(engine, vec![])
        .unwrap();
        let orig_bytes = b.rlp_bytes();
        let orig_db = b.drain().state.drop().1;

        let db = spec
            .ensure_db_good(get_temp_state_db(), &Default::default())
            .unwrap();
        let e = enact_and_seal(
            orig_bytes.clone(),
            engine,
            false,
            db,
            &genesis_header,
            last_hashes,
            Default::default(),
        )
        .unwrap();

        assert_eq!(e.rlp_bytes(), orig_bytes);

        let db = e.drain().state.drop().1;
        assert_eq!(orig_db.journal_db().keys(), db.journal_db().keys());
        assert!(
            orig_db
                .journal_db()
                .keys()
                .iter()
                .filter(|k| orig_db.journal_db().get(k.0) != db.journal_db().get(k.0))
                .next()
                == None
        );
    }

    #[test]
    fn enact_block_with_uncle() {
        use spec::*;
        let spec = Spec::new_test();
        let engine = &*spec.engine;
        let genesis_header = spec.genesis_header();

        let db = spec
            .ensure_db_good(get_temp_state_db(), &Default::default())
            .unwrap();
        let last_hashes = Arc::new(vec![genesis_header.hash()]);
        let mut open_block = OpenBlock::new(
            engine,
            Default::default(),
            false,
            db,
            &genesis_header,
            last_hashes.clone(),
            Address::zero(),
            (3141562.into(), 31415620.into()),
            vec![],
            false,
            None,
        )
        .unwrap();
        let mut uncle1_header = Header::new();
        uncle1_header.set_extra_data(b"uncle1".to_vec());
        let mut uncle2_header = Header::new();
        uncle2_header.set_extra_data(b"uncle2".to_vec());
        open_block.push_uncle(uncle1_header).unwrap();
        open_block.push_uncle(uncle2_header).unwrap();
        let b = open_block
            .close_and_lock()
            .unwrap()
            .seal(engine, vec![])
            .unwrap();

        let orig_bytes = b.rlp_bytes();
        let orig_db = b.drain().state.drop().1;

        let db = spec
            .ensure_db_good(get_temp_state_db(), &Default::default())
            .unwrap();
        let e = enact_and_seal(
            orig_bytes.clone(),
            engine,
            false,
            db,
            &genesis_header,
            last_hashes,
            Default::default(),
        )
        .unwrap();

        let bytes = e.rlp_bytes();
        assert_eq!(bytes, orig_bytes);
        let uncles = view!(BlockView, &bytes).uncles(engine.params().eip1559_transition);
        assert_eq!(uncles[1].extra_data(), b"uncle2");

        let db = e.drain().state.drop().1;
        assert_eq!(orig_db.journal_db().keys(), db.journal_db().keys());
        assert!(
            orig_db
                .journal_db()
                .keys()
                .iter()
                .filter(|k| orig_db.journal_db().get(k.0) != db.journal_db().get(k.0))
                .next()
                == None
        );
    }
}
