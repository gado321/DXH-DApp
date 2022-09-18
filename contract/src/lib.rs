use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen, env, AccountId, Promise, Gas, Balance};
use json::{parse};
use serde_json::json;

const DEPOSIT: u128 = 1;
const CALL_GAS: Gas = Gas(5_000_000_000_000);

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    candidates: Vec<String>,
    verified_candidates: Vec<String>
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            candidates: vec![],
            verified_candidates: vec![]
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    // candidates
    pub fn set_candidate(&mut self, candidate: String) {
        parse(&candidate).expect("Wrong format!");

        self.candidates.push(candidate);
    }
    
    pub fn get_candidates(&mut self) -> Vec<String> {
        self.candidates.clone()
    }

    pub fn remove_candidate(&mut self, candidate: String) {
        self.candidates.retain(|x| * x != candidate);
    }
    

    // verified candidates
    pub fn set_verified_candidate(&mut self, candidate: String) {
        let account_id = env::signer_account_id();
        
        if account_id.to_string() == "upi05.testnet".to_string() {
            // Set MAX_COIN before pushing
            self.verified_candidates.push(candidate.clone());
            self.remove_candidate(candidate);

        }        
    }
    
    pub fn get_verified_candidates(&mut self) -> Vec<String> {
        self.verified_candidates.clone()
    }

    pub fn remove_verified_candidate(&mut self, candidate: String) {
        let account_id = env::signer_account_id();
        if account_id.to_string() == "upi05.testnet".to_string() {
            self.verified_candidates.retain(|x| * x != candidate);
        }
    }

    // donate
    // Call this function to trigger token widthdraw process from donation pool
    pub fn donate_trigger(&self, amount: String) {
        let args = json!({
            "receiver_id": "test1.upi05.testnet".to_string(),
            "amount":  amount
        }) .to_string().into_bytes().to_vec();

        Promise::new("dev-1663407143254-90994928167650".parse().unwrap())
        .function_call("ft_transfer".to_string(), args, DEPOSIT, CALL_GAS);
    }
}


// Unit test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_and_set_candidates_testing() {
        let mut contract = Contract::default();
        let candidate: String = r#"

        {
            "code": 200,
            "success": true,
            "payload": {
                "features": [
                    "awesome",
                    "easyAPI",
                    "lowLearningCurve"
                ]
            }
        }
        
        "#.to_owned();
        contract.set_candidate(candidate.clone());
        assert_eq!(
            contract.get_candidates()[0],
            candidate
        );
    }
    
}
