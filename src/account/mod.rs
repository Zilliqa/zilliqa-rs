pub mod error;

use crate::crypto::util::{
    get_address_from_public_key, get_pub_key_from_private_key, normalize_private_key,
};

use self::error::AccountError;

pub struct Account {
    private_key: String,
    public_key: String,
    address: String,
    bech32_address: String,
}

impl Account {
    pub fn new(private_key: String) -> Result<Self, AccountError> {
        let private_key = normalize_private_key(&private_key)?;
        let public_key = get_pub_key_from_private_key(&private_key)?;
        let address = get_address_from_public_key(&public_key)?;

        Ok(Self {
            private_key,
            public_key,
            address,
            bech32_address: "".to_string(),
        })
    }
}
