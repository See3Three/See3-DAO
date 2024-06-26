#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod See3_DAO {
    use ink::primitives::Key;
    use ink::{
        storage::Mapping,
    };

    type TrustKey = [[u8; 32]; 2];


    #[derive(PartialEq, Debug, scale::Decode, scale::Encode)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum VoteType {
        None,
        ChangeKeeper(AccountId, AccountId),
        AddToTrustList(AccountId, TrustKey),
        RemoveFromTrustList(AccountId),
        SlashParticipant(AccountId),
    }

    #[ink(storage)]
    pub struct See3Dao {
        total_supply: u32,
        vote: VoteType,
        vote_end_block: u32,
        earliest_next_vote_block: u32,
        voters: Mapping<AccountId, bool>, 
        casted_votes: u32, 
        keepers: [AccountId; 3],
        trust_list: Mapping<AccountId, TrustKey>,
        balances: Mapping<AccountId, (u32, u32)>,
        withdrawable: Mapping<AccountId, u32>,
    }

    impl See3Dao {
        #[ink(constructor)]
        pub fn new(supply: u32, initial_keepers: [AccountId; 3]) -> Self {
            let caller = Self::env().caller();
            let mut vote = VoteType::None;
            let mut vote_end_block = 0;
            let mut earliest_next_vote_block = 0;
            let mut casted_votes = 0;
            let mut voters = Mapping::default();
            let mut keepers = initial_keepers;
            let mut trust_list = Mapping::default();
            let mut balances = Mapping::default();    
            let mut withdrawable = Mapping::default();
            balances.insert(&caller, &(supply, supply));
            Self {
                total_supply: supply,
                vote,
                vote_end_block,
                earliest_next_vote_block,
                casted_votes,
                voters,
                keepers,
                trust_list,
                balances,
                withdrawable,
            }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> u32 {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, account: AccountId) -> u32 {
            self.balances.get(&account).map_or(0, |balance_tuple| balance_tuple.0)
        }

        #[ink(message)]
        pub fn voting_power_of(&self, account: AccountId) -> u32 {
            self.balances.get(&account).map_or(0, |balance_tuple| balance_tuple.1)
        }

        #[ink(message)]
        pub fn deposit_of(&self, account: AccountId) -> u32 {
            self.withdrawable.get(&account).unwrap_or(0)
        }

        #[ink(message)]
        pub fn transfer(&mut self, recipient: AccountId, amount: u32) {
            let sender = self.env().caller();
            let sender_balance = self.balance_of(sender);
            if sender_balance < amount {
                return;
            }
            if !(self.vote == VoteType::None) {
                let sender_voting_power = self.voting_power_of(sender);
                let updated_sender_balance = sender_balance.checked_sub(amount).expect("Overflow in balance subtraction");
                self.balances.insert(sender, &(updated_sender_balance, sender_voting_power));
                // Update Recipient Balance, but Keep Snapshot The Same
                let recipient_balance = self.balance_of(recipient);
                let recipient_voting_power = self.voting_power_of(recipient);
                let updated_recipient_balance = recipient_balance.checked_add(amount).expect("Overflow in balance addition");
                self.balances.insert(recipient, &(updated_recipient_balance, recipient_voting_power));
            } else {
                let updated_sender_balance = sender_balance.checked_sub(amount).expect("Overflow in balance subtraction");
                self.balances.insert(sender, &(updated_sender_balance, updated_sender_balance));
                // Update Recipient Balance, and Snapshot
                let recipient_balance = self.balance_of(recipient);
                let updated_recipient_balance = recipient_balance.checked_add(amount).expect("Overflow in balance addition");
                self.balances.insert(recipient, &((updated_recipient_balance), (updated_recipient_balance)));
            }
        }

        #[ink(message)]
        pub fn initialize_vote(&mut self, vote: VoteType) {
            let current_block = self.env().block_number();
            if (self.vote == VoteType::None) && (current_block > self.earliest_next_vote_block) {
                self.vote = vote;
                self.casted_votes = 0;
                self.vote_end_block = current_block.checked_add(1000).expect("Overflow in vote_end_block calculation");
                self.earliest_next_vote_block = self.vote_end_block.checked_add(1000).expect("Overflow in earliest_next_vote_block calculation");
            }
        }

        #[ink(message)]
        pub fn cast_vote(&mut self, in_favor: bool) {
            let current_block = self.env().block_number();
            let sender = self.env().caller();
            let has_voted = self.voters.get(&sender).unwrap_or(false);
            if !has_voted && (current_block < self.vote_end_block) {
                if in_favor {
                    let sender_voting_power = self.voting_power_of(sender);
                    self.casted_votes = self.casted_votes.checked_add(sender_voting_power).expect("Overflow in adding votes");
                }
                self.voters.insert(&sender, &true);
            } else {
                return;
            }
        }

        #[ink(message)]
        pub fn finalize_vote(&mut self) {
            let current_block = self.env().block_number();
            if (current_block > self.vote_end_block) && !(self.vote == VoteType::None) {
                if self.casted_votes > self.total_supply / 2 {
                    match self.vote {
                        VoteType::ChangeKeeper(old_keeper, new_keeper) => {
                            for i in 0..self.keepers.len() {
                                if self.keepers[i] == old_keeper {
                                    self.keepers[i] = new_keeper;
                                    break;
                                }
                            }
                        },
                        VoteType::AddToTrustList(account, trust_keys) => {
                            if self.withdrawable.get(account).unwrap_or(0) > 100 {
                                self.trust_list.insert(account, &trust_keys);
                            }
                        },
                        VoteType::RemoveFromTrustList(account) => {
                            self.trust_list.remove(account);
                        },
                        VoteType::SlashParticipant(account) => {
                            self.withdrawable.remove(account);
                        },
                        VoteType::None => {}
                    }
                }
                self.clear_vote_state();
            }
        }

        #[ink(message, payable)]
        pub fn deposit(&mut self) {
            let sender = self.env().caller();
            let deposit: u32 = self.env().transferred_value().try_into().unwrap();
            self.withdrawable.insert(&sender, &deposit);
        }

        #[ink(message)]
        pub fn withdraw(&mut self) {
            let sender = self.env().caller();
            let deposit = self.deposit_of(sender);
            self.env().transfer(sender, deposit.try_into().unwrap()).expect("Withdrawal Failed.");
            self.withdrawable.remove(&sender);
        }

        fn clear_vote_state(&mut self) {
            self.casted_votes = 0;
            self.voters = Mapping::default();
            self.vote = VoteType::None;
        }
    }
}
