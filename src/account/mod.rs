pub mod error;
pub mod transaction;
pub mod wallet;

pub use transaction::Transaction;

use crate::crypto::{
    bech32::to_bech32_address,
    util::{get_address_from_public_key, get_pub_key_from_private_key, normalize_private_key},
};

use self::error::AccountError;

#[derive(PartialEq, Debug, Clone)]
pub struct Account {
    pub private_key: String,
    pub public_key: String,
    pub address: String,
    pub bech32_address: String,
}

impl Account {
    pub fn new(private_key: &str) -> Result<Self, AccountError> {
        let private_key = normalize_private_key(private_key)?;
        let public_key = get_pub_key_from_private_key(&private_key)?;
        let address = get_address_from_public_key(&public_key)?;
        let bech32_address = to_bech32_address(&address)?;

        Ok(Self {
            private_key,
            public_key,
            address,
            bech32_address,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Account;

    #[test]
    fn a_valid_private_key_should_results_a_valid_account() {
        let account =
            Account::new("0xD96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba")
                .unwrap();
        assert_eq!(
            account,
            Account {
                private_key: String::from(
                    "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba"
                ),
                public_key: String::from(
                    "03bfad0f0b53cff5213b5947f3ddd66acee8906aba3610c111915aecc84092e052"
                ),
                address: String::from("0x381f4008505e940AD7681EC3468a719060caF796"),
                bech32_address: String::from("zil18q05qzzst62q44mgrmp5dzn3jpsv4aukxredu2")
            }
        );
    }
}
