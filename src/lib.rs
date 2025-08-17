use near_contract_standards::non_fungible_token::{
    NonFungibleToken, Token, metadata::{NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC},
    approval::NonFungibleTokenApproval,
    enumeration::NonFungibleTokenEnumeration,
    core::NonFungibleTokenCore,
};
use near_sdk::collections::LazyOption;
use near_sdk::{near, require, env, AccountId, BorshStorageKey, PanicOnDefault, PromiseOrValue};
use near_sdk::borsh::{self, BorshSerialize};

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
}

#[near]
impl Contract {
    /// Khởi tạo với metadata mặc định
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        let metadata = NFTContractMetadata {
            spec: NFT_METADATA_SPEC.to_string(),
            name: "Knight LEGACY NFT ".to_string(),
            symbol: "KLN".to_string(),
            icon: None,
            base_uri: None,
            reference: None,
            reference_hash: None,
        };
        Self::new(owner_id, metadata)
    }

    /// Khởi tạo đầy đủ metadata
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        require!(!env::state_exists(), "Already initialized");
        let this: Contract = Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
        };
        this
    }

    /// Mint 1 NFT (đính kèm đủ deposit để chi trả storage)
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: String,
        receiver_id: AccountId,
        token_metadata: TokenMetadata,
    ) -> Token {
        // Người gọi trả phí lưu trữ; NonFungibleToken đã tính toán & hoàn refund phần dư
        self.tokens.internal_mint(token_id.into(), receiver_id, Some(token_metadata))
    }
}

// Triển khai các tiêu chuẩn NEP-171/177/178 (core/approval/enumeration)
#[near]
impl NonFungibleTokenCore for Contract {
    // ủy quyền cho implementation có sẵn
    fn nft_transfer(&mut self, receiver_id: AccountId, token_id: String, approval_id: Option<u64>, memo: Option<String>) {
        self.tokens.nft_transfer(receiver_id, token_id, approval_id, memo)
    }
    fn nft_transfer_call(&mut self, receiver_id: AccountId, token_id: String, approval_id: Option<u64>, memo: Option<String>, msg: String) -> PromiseOrValue<bool> {
        self.tokens.nft_transfer_call(receiver_id, token_id, approval_id, memo, msg)
    }
    fn nft_token(&self, token_id: String) -> Option<Token> {
        self.tokens.nft_token(token_id)
    }
}

#[near]
impl NonFungibleTokenApproval for Contract {
    fn nft_approve(
        &mut self,
        token_id: String,
        account_id: AccountId,
        msg: Option<String>
    ) -> Option<near_sdk::Promise> {
        self.tokens.nft_approve(token_id, account_id, msg)
    }

    fn nft_revoke(&mut self, token_id: String, account_id: AccountId) {
        self.tokens.nft_revoke(token_id, account_id)
    }

    fn nft_revoke_all(&mut self, token_id: String) {
        self.tokens.nft_revoke_all(token_id)
    }

    fn nft_is_approved(
        &self,
        token_id: String,
        approved_account_id: AccountId,
        approval_id: Option<u64>
    ) -> bool {
        self.tokens.nft_is_approved(token_id, approved_account_id, approval_id)
    }
}

#[near]
impl NonFungibleTokenEnumeration for Contract {
    fn nft_total_supply(&self) -> near_sdk::json_types::U128 {
        self.tokens.nft_total_supply()
    }

    fn nft_tokens(
        &self,
        from_index: Option<near_sdk::json_types::U128>,
        limit: Option<u64>
    ) -> Vec<Token> {
        self.tokens.nft_tokens(from_index, limit)
    }

    fn nft_supply_for_owner(&self, account_id: AccountId) -> near_sdk::json_types::U128 {
        self.tokens.nft_supply_for_owner(account_id)
    }

    fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<near_sdk::json_types::U128>,
        limit: Option<u64>
    ) -> Vec<Token> {
        self.tokens.nft_tokens_for_owner(account_id, from_index, limit)
    }
}


#[near]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}
