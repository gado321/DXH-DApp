/*!
Fungible Token implementation with JSON serialization.
NOTES:
  - The maximum balance value is limited by U128 (2**128 - 1).
  - JSON calls should pass U128 as a base-10 string. E.g. "100".
  - The contract optimizes the inner trie structure by hashing account IDs. It will prevent some
    abuse of deep tries. Shouldn't be an issue, once NEAR clients implement full hashing of keys.
  - The contract tracks the change in storage before and after the call. If the storage increases,
    the contract requires the caller of the contract to attach enough deposit to the function call
    to cover the storage cost.
    This is done to prevent a denial of service attack on the contract by taking all available storage.
    If the storage decreases, the contract will issue a refund for the cost of the released storage.
    The unused tokens from the attached deposit are also refunded, so it's safe to
    attach more deposit than required.
  - To prevent the deployed contract from being modified or deleted, it should not have any access
    keys on its account.
*/
use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::U128;
use near_sdk::{env, log, near_bindgen, AccountId, Balance, PanicOnDefault, PromiseOrValue};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3c%3fxml version='1.0' encoding='iso-8859-1'%3f%3e %3c!-- Generator: Adobe Illustrator 19.0.0%2c SVG Export Plug-In . SVG Version: 6.00 Build 0) --%3e %3csvg version='1.1' id='Capa_1' xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink' x='0px' y='0px' viewBox='0 0 58 58' style='enable-background:new 0 0 58 58%3b' xml:space='preserve'%3e%3cg id='XMLID_90_'%3e%3cpath id='XMLID_122_' style='fill:%23F4BF1A%3b' d='M43.918%2c44.203C33.805%2c54.316%2c18.832%2c56.016%2c9.069%2c48.646 c0.599%2c0.793%2c1.25%2c1.556%2c1.969%2c2.275c9.567%2c9.567%2c26.188%2c8.459%2c37.123-2.475c10.934-10.935%2c12.042-27.556%2c2.474-37.123 c-0.718-0.719-1.481-1.37-2.274-1.969C55.731%2c19.117%2c54.031%2c34.09%2c43.918%2c44.203'/%3e%3cpath id='XMLID_121_' style='fill:%23FFD949%3b' d='M46.393%2c7.08c9.568%2c9.568%2c8.46%2c26.188-2.475%2c37.123 C32.983%2c55.138%2c16.363%2c56.246%2c6.795%2c46.678C-2.773%2c37.11-1.665%2c20.49%2c9.27%2c9.555C20.205-1.38%2c36.825-2.488%2c46.393%2c7.08'/%3e%3cpath id='XMLID_89_' style='fill:%23F4BF1A%3b' d='M23.091%2c49.188c-5.243%2c0-10.025-1.896-13.468-5.338 c-7.993-7.993-6.883-22.109%2c2.475-31.466c4.965-4.966%2c11.525-7.813%2c17.998-7.813c5.243%2c0%2c10.025%2c1.896%2c13.468%2c5.338 c7.993%2c7.993%2c6.883%2c22.109-2.475%2c31.466C36.125%2c46.339%2c29.565%2c49.188%2c23.091%2c49.188'/%3e%3cpath id='XMLID_119_' style='fill:%23DCA815%3b' d='M44.596%2c43.467l4.256%2c4.256c0.461-0.49%2c0.906-0.989%2c1.329-1.499l-4.249-4.249 C45.509%2c42.484%2c45.056%2c42.977%2c44.596%2c43.467'/%3e%3cpath id='XMLID_118_' style='fill:%23DCA815%3b' d='M40.895%2c46.837l4.258%2c4.258c0.525-0.407%2c1.039-0.837%2c1.544-1.284l-4.251-4.251 C41.939%2c46.007%2c41.421%2c46.429%2c40.895%2c46.837'/%3e%3cpath id='XMLID_117_' style='fill:%23DCA815%3b' d='M47.746%2c39.545l4.255%2c4.255c0.386-0.57%2c0.752-1.148%2c1.095-1.733l-4.246-4.246 C48.506%2c38.406%2c48.131%2c38.978%2c47.746%2c39.545'/%3e%3cpath id='XMLID_116_' style='fill:%23DCA815%3b' d='M50.29%2c35.018l4.26%2c4.259c0.301-0.662%2c0.573-1.331%2c0.822-2.005l-4.254-4.254 C50.869%2c33.691%2c50.59%2c34.357%2c50.29%2c35.018'/%3e%3cpath id='XMLID_115_' style='fill:%23DCA815%3b' d='M36.62%2c49.633l4.256%2c4.256c0.603-0.326%2c1.201-0.668%2c1.789-1.04l-4.259-4.258 C37.818%2c48.962%2c37.223%2c49.307%2c36.62%2c49.633'/%3e%3cpath id='XMLID_114_' style='fill:%23DCA815%3b' d='M31.683%2c51.767l4.255%2c4.255c0.693-0.228%2c1.384-0.478%2c2.068-0.761l-4.254-4.253 C33.068%2c51.29%2c32.378%2c51.54%2c31.683%2c51.767'/%3e%3cpath id='XMLID_113_' style='fill:%23DCA815%3b' d='M24.555%2c53.125l4.259%2c4.259c0.851-0.048%2c1.706-0.137%2c2.561-0.267l-4.261-4.261 C26.259%2c52.985%2c25.406%2c53.078%2c24.555%2c53.125'/%3e%3cpath id='XMLID_112_' style='fill:%23DCA815%3b' d='M16.688%2c52.328l4.27%2c4.27c1.14%2c0.306%2c2.306%2c0.527%2c3.49%2c0.663l-4.268-4.269 C18.995%2c52.854%2c17.829%2c52.636%2c16.688%2c52.328'/%3e%3cpath id='XMLID_111_' style='fill:%23DCA815%3b' d='M52.368%2c28.611l4.252%2c4.252c0.162-0.818%2c0.287-1.637%2c0.373-2.455l-4.257-4.257 C52.65%2c26.971%2c52.529%2c27.791%2c52.368%2c28.611'/%3e%3cpath id='XMLID_88_' style='fill:%23DCA815%3b' d='M57.103%2c26.275c-0.057-1.079-0.176-2.148-0.37-3.198l-4.281-4.281 c0.199%2c1.065%2c0.352%2c2.142%2c0.409%2c3.237L57.103%2c26.275z'/%3e%3cpath id='XMLID_109_' style='fill:%23D1D4D2%3b' d='M5.715%2c57c-2.757%2c0-5-2.243-5-5c0-0.552%2c0.448-1%2c1-1c0.552%2c0%2c1%2c0.448%2c1%2c1 c0%2c1.654%2c1.346%2c3%2c3%2c3c0.552%2c0%2c1%2c0.448%2c1%2c1C6.715%2c56.552%2c6.267%2c57%2c5.715%2c57'/%3e%3cpath id='XMLID_108_' style='fill:%23D1D4D2%3b' d='M55.715%2c7c-0.552%2c0-1-0.448-1-1c0-2.206-1.346-4-3-4c-0.552%2c0-1-0.448-1-1 c0-0.552%2c0.448-1%2c1-1c2.757%2c0%2c5%2c2.691%2c5%2c6C56.715%2c6.552%2c56.267%2c7%2c55.715%2c7'/%3e%3cpath id='XMLID_107_' style='fill:%23D1D4D2%3b' d='M45.715%2c56c-0.552%2c0-1-0.448-1-1c0-0.552%2c0.448-1%2c1-1c4.962%2c0%2c9-4.038%2c9-9 c0-0.552%2c0.448-1%2c1-1c0.552%2c0%2c1%2c0.448%2c1%2c1C56.715%2c51.065%2c51.78%2c56%2c45.715%2c56'/%3e%3cpath id='XMLID_106_' style='fill:%23D1D4D2%3b' d='M52.715%2c58c-0.552%2c0-1-0.448-1-1c0-0.552%2c0.448-1%2c1-1c1.626%2c0%2c3-0.916%2c3-2 c0-0.552%2c0.448-1%2c1-1c0.552%2c0%2c1%2c0.448%2c1%2c1C57.715%2c56.206%2c55.472%2c58%2c52.715%2c58'/%3e%3cpath id='XMLID_105_' style='fill:%23FFD949%3b' d='M38.226%2c22.875c-0.239-5.821-4.527-8.069-11.098-8.076 c-0.291%2c0-0.577%2c0.009-0.858%2c0.028c-2.047%2c0.135-3.953-0.19-5.331-1.122c-0.556-0.377-1-0.772-1.281-1.185 c-0.127-0.188-0.48-0.058-0.499%2c0.182c-0.032%2c0.414-0.054%2c0.899-0.046%2c1.433c0.028%2c2.023-0.173%2c1.722-1.405%2c3.631 c-0.681%2c1.056-1.225%2c2.24-1.63%2c3.513c-0.896%2c0.366-1.625%2c1.151-1.834%2c2.117l-0.276%2c1.281c-0.25%2c1.158%2c0.302%2c2.248%2c1.291%2c2.733 c0.151%2c2.882%2c1.216%2c5.46%2c3.774%2c7.265l-0.321%2c1.742c-0.083%2c0.45-0.408%2c0.802-0.835%2c0.907l-5.97%2c1.455 c-1.552%2c0.456-2.739%2c1.821-3.131%2c3.59L8.683%2c42.79c0.305%2c0.358%2c0.603%2c0.722%2c0.941%2c1.059c3.442%2c3.443%2c8.225%2c5.339%2c13.468%2c5.339 c4.467%2c0%2c8.976-1.358%2c12.974-3.831l-6.81-5.255c-0.446-0.345-0.681-0.872-0.611-1.368l0.273-1.958 c3.822-0.673%2c6.344-3.058%2c7.814-6.34c1.714-0.197%2c3.097-1.321%2c3.242-2.841l0.159-1.674C40.254%2c24.656%2c39.452%2c23.508%2c38.226%2c22.875'/%3e%3c/g%3e%3cg%3e%3c/g%3e%3cg%3e%3c/g%3e%3cg%3e%3c/g%3e%3cg%3e%3c/g%3e%3cg%3e%3c/g%3e%3cg%3e%3c/g%3e%3cg%3e%3c/g%3e%3cg%3e%3c/g%3e%3cg%3e%3c/g%3e%3cg%3e%3c/g%3e%3cg%3e%3c/g%3e%3cg%3e%3c/g%3e%3cg%3e%3c/g%3e%3cg%3e%3c/g%3e%3cg%3e%3c/g%3e%3c/svg%3e";

#[near_bindgen]
impl Contract {
    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// default metadata.
    #[init]
    pub fn new_default_meta(owner_id: AccountId, total_supply: U128) -> Self {
        Self::new(
            owner_id,
            total_supply,
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "DayXaHoi".to_string(),
                symbol: "DXH".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                reference: None,
                reference_hash: None,
                decimals: 18,
            },
        )
    }

    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given fungible token metadata.
    #[init]
    pub fn new(
        owner_id: AccountId,
        total_supply: U128,
        metadata: FungibleTokenMetadata,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        let mut this = Self {
            token: FungibleToken::new(b"a".to_vec()),
            metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
        };
        this.token.internal_register_account(&owner_id);
        this.token.internal_deposit(&owner_id, total_supply.into());
        near_contract_standards::fungible_token::events::FtMint {
            owner_id: &owner_id,
            amount: &total_supply,
            memo: Some("Initial tokens supply is minted"),
        }
        .emit();
        this
    }

    fn on_account_closed(&mut self, account_id: AccountId, balance: Balance) {
        log!("Closed @{} with {}", account_id, balance);
    }

    fn on_tokens_burned(&mut self, account_id: AccountId, amount: Balance) {
        log!("Account @{} burned {}", account_id, amount);
    }
}

near_contract_standards::impl_fungible_token_core!(Contract, token, on_tokens_burned);
near_contract_standards::impl_fungible_token_storage!(Contract, token, on_account_closed);

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, Balance};

    use super::*;

    const TOTAL_SUPPLY: Balance = 1_000_000_000_000_000;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new_default_meta(accounts(1).into(), TOTAL_SUPPLY.into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.ft_total_supply().0, TOTAL_SUPPLY);
        assert_eq!(contract.ft_balance_of(accounts(1)).0, TOTAL_SUPPLY);
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }

    #[test]
    fn test_transfer() {
        let mut context = get_context(accounts(2));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(2).into(), TOTAL_SUPPLY.into());
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(contract.storage_balance_bounds().min.into())
            .predecessor_account_id(accounts(1))
            .build());
        // Paying for account registration, aka storage deposit
        contract.storage_deposit(None, None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(2))
            .build());
        let transfer_amount = TOTAL_SUPPLY / 3;
        contract.ft_transfer(accounts(1), transfer_amount.into(), None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert_eq!(contract.ft_balance_of(accounts(2)).0, (TOTAL_SUPPLY - transfer_amount));
        assert_eq!(contract.ft_balance_of(accounts(1)).0, transfer_amount);
    }
}
