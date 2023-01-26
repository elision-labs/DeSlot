use near_sdk::{env, near_bindgen, U128};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct AdSlot {
    owner_id: String,
    highest_bidder_id: String,
    highest_bid: U128,
    escrow_account_id: String,
    escrow_amount: U128,
}

#[near_bindgen]
impl AdSlot {
    pub fn new(owner_id: String, escrow_account_id: String) -> Self {
        Self {
            owner_id,
            highest_bidder_id: "".to_string(),
            highest_bid: 0,
            escrow_account_id,
            escrow_amount: 0,
        }
    }

    #[payable]
    pub fn bid(&mut self, bidder_id: String, bid: U128) {
        if bid > self.highest_bid {
            self.highest_bid = bid;
            self.highest_bidder_id = bidder_id;
            let escrow_amount = bid;

            env::transfer_to_account(self.escrow_account_id.clone(), escrow_amount);
            self.escrow_amount = escrow_amount;
        }
    }

    pub fn release_funds(&mut self) {
        let highest_bidder_id = self.highest_bidder_id.clone();
        let escrow_amount = self.escrow_amount;
        env::transfer_to_account(highest_bidder_id, escrow_amount);
        self.escrow_amount = 0;
    }

    pub fn get_highest_bid(&self) -> U128 {
        self.highest_bid
    }

    pub fn get_highest_bidder_id(&self) -> String {
        self.highest_bidder_id.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(predecessor_account_id: &str) -> VMContext {
        VMContext {
            current_account_id: "elision_test".to_string(),
            signer_account_id: "elision_test".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: predecessor_account_id.to_string(),
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
        }
    }

    #[test]
    fn test_bid() {
        let context = get_context("alice_test");
        testing_env!(context);

        let mut ad_space = AdSlot::new("alice_test".to_string(), "escrow_test".to_string());
        ad_space.bid("bob_test".to_string(), 100);
        assert_eq!(ad_space.get_highest_bid(), 100);
        assert_eq!(ad_space.get_highest_bidder_id(), "bob_test".to_string());
    }
    #[test]
    fn test_release_funds() {
        let context = get_context("alice_test");
        testing_env!(context);

        let mut ad_space = AdSlot::new("alice_test".to_string(), "escrow_test".to_string());
        ad_space.bid("bob_test".to_string(), 100);

        let balance_before = env::attached_deposit();
        ad_space.release_funds();

        let balance_after = env::attached_deposit();
        assert_eq!(balance_before - 100, balance_after);
    }
}
