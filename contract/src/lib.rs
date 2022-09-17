/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

 use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
 use near_sdk::{log, near_bindgen};
 
 // Define the default candidate
 const DEFAULT_CANDIDATE: &str = &"DXH";
 
 // Define the contract structure
 #[near_bindgen]
 #[derive(BorshDeserialize, BorshSerialize)]
 pub struct Contract {
     candidates: Vec<String>
 }
 
 // Define the default, which automatically initializes the contract
 impl Default for Contract {
     fn default() -> Self{
         Self{candidates: vec![DEFAULT_CANDIDATE.to_string()]}
     }
 }
 
 // Implement the contract structure
 impl Contract {
     fn set_candidates(&mut self, candidate: String) {
         // Verify json structure
 
         //
         self.candidates.push(candidate);
     }
 
     fn get_candidates(&mut self) -> Vec<String> {
         self.candidates.clone()
     }
     
     fn remove_candidate(&mut self, candidate: String) {
         self.candidates.retain(|x| * x != candidate);
     }
     
 }
 
 
 /*
  * The rest of this file holds the inline tests for the code above
  * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
  */
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn get_candidates_testing() {
         let mut contract = Contract::default();
         assert_eq!(
             contract.get_candidates()[0],
             DEFAULT_CANDIDATE
         );
     }
     #[test]
     fn set_candidates_testing() {
         let mut contract = Contract::default();
         let tmp: String = "dxh".to_owned();
         contract.set_candidates(tmp);
         assert_eq!(
             contract.get_candidates()[1],
             "dxh".to_owned()
         );
     }
 }
 