use near_sdk::json_types::U64;

use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]

    pub fn create_series(
        &mut self,
        id: u64,
        metadata: TokenMetadata,
        royalty: Option<HashMap<AccountId, u32>>,
        price: Option<U128>,
    ) {
        let initial_storage_usage = env::storage_usage();

        let caller = env::predecessor_account_id();
        require!(
            self.approved_creators.contains(&caller),
            "only approved creators can add a type"
        );

        require!(
            self.series_by_id
                .insert(
                    &id,
                    &Series {
                        metadata,
                        royalty,
                        tokens: UnorderedSet::new(StorageKey::SeriesByIdInner {
                            account_id_hash: hash_account_id(&format!("{}{}", id, caller)),
                        }),
                        owner_id: caller,
                        price: price.map(|p| p.into()),
                    }
                )
                .is_none(),
            "collection ID already exists"
        );

        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        refund_deposit(required_storage_in_bytes);
    }

    #[payable]
    pub fn nft_mint(&mut self, id: U64, receiver_id: AccountId) {
        let initial_storage_usage = env::storage_usage();

        let mut series = self.series_by_id.get(&id.0).expect("Not a series");

        let mut price_per_token = 0;
        if let Some(price) = series.price {
            price_per_token = price;
            require!(
                env::attached_deposit() > price_per_token,
                "Need to attach at least enough to cover price"
            );
        } else {
            let predecessor = env::predecessor_account_id();
            assert!(
                self.approved_minters.contains(&predecessor),
                "Not approved minter"
            );
        }

        let cur_len = series.tokens.len();

        if let Some(copies) = series.metadata.copies {
            require!(
                cur_len < copies,
                "cannot mint anymore NFTs for the given series. Limit reached"
            );
        }

        let token_id = format!("{}:{}", id.0, cur_len + 1);
        series.tokens.insert(&token_id);
        self.series_by_id.insert(&id.0, &series);

        let token = Token {
            series_id: id.0,
            owner_id: receiver_id,
            approved_account_ids: Default::default(),
            next_approval_id: 0,
        };

        require!(
            self.tokens_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );

        self.internal_add_token_to_owner(&token.owner_id, &token_id);

        let nft_mint_log: EventLog = EventLog {
            standard: NFT_STANDARD_NAME.to_string(),
            version: NFT_METADATA_SPEC.to_string(),
            event: EventLogVariant::NftMint(vec![NftMintLog {
                owner_id: token.owner_id.to_string(),
                token_ids: vec![token_id.to_string()],
                memo: None,
            }]),
        };

        env::log_str(&nft_mint_log.to_string());

        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        if price_per_token > 0 {
            payout_series_owner(required_storage_in_bytes, price_per_token, series.owner_id);
        } else {
            refund_deposit(required_storage_in_bytes);
        }
    }
}
