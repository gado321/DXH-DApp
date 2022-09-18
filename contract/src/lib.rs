use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen, env, AccountId, PromiseError, Promise, Gas, Balance};
use json::{parse};
use serde_json::{Value, json, from_str};

const DEPOSIT: u128 = 3;
const CALL_GAS: Gas = Gas(10_000_000_000_000);

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    candidates: Vec<String>,
    verified_candidates: Vec<String>,
    pool_balance: u128
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            candidates: vec![],
            verified_candidates: vec![],
            pool_balance: 0
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    // pool balance
    #[private]
    pub fn get_pool_balance(&self) {
        let args = json!({
            "account_id": env::current_account_id()
        }) .to_string().into_bytes().to_vec();

        let promise = Promise::new("dev-1663407143254-90994928167650".parse().unwrap())
            .function_call("ft_balance_of".to_string(), args.clone(), 1, CALL_GAS);
        promise.then(
            Promise::new(env::current_account_id())
            .function_call("get_pool_balance_callback".to_string(), args, 0, CALL_GAS)
        );
    }

    #[private]
    pub fn get_pool_balance_callback(&mut self, #[callback_result] call_result: Result<String, PromiseError>) {
        // Check if the promise succeeded by calling the method outlined in external.rs
        if call_result.is_err() {
          log!("There was an error contacting NEAR");
        } else {
            let res: String = call_result.unwrap();
            self.pool_balance = res.parse().unwrap();
        }
    }

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
    pub fn donate(&mut self) {
        
        let candidate: Value = serde_json::from_str(&self.verified_candidates[0]).unwrap();
        
        let verified_candidates: Vec<String> = self.get_verified_candidates();
        
        for i in 0..verified_candidates.len() {
            self.get_pool_balance();

            let candidate: Value = serde_json::from_str(&verified_candidates[i]).unwrap();

            let donatedAmount: u128 = candidate["donatedAmount"].to_string().parse().unwrap();

            if donatedAmount + 1000 <= self.pool_balance {
                let args = json!({
                    "receiver_id": candidate["publicKey"].to_string().replace("\"", ""),
                    "amount": candidate["donatedAmount"].to_string()
                }).to_string().into_bytes().to_vec();
                Promise::new("dev-1663407143254-90994928167650".parse().unwrap())
                .function_call("ft_transfer".to_string(), args, 1, CALL_GAS);

                self.remove_verified_candidate(verified_candidates[i].clone());
            } else {
                break;
            }
        }
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
