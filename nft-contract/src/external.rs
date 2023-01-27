use near_sdk::{ext_contract};

// Validator interface, for cross-contract calls

#[ext_contract(ext_self)]
pub trait NftContract {
    fn my_callback(&self) -> String;
}
