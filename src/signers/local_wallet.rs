use std::str::FromStr;

use k256::ecdsa::Signature;

use crate::{
    crypto::{generate_private_key, schnorr::sign, PrivateKey, PublicKey, ZilAddress},
    Error,
};

use super::Signer;

#[derive(Debug, Clone, PartialEq)]
pub struct LocalWallet {
    pub private_key: PrivateKey,
    pub address: ZilAddress,
    public_key: PublicKey,
}

impl LocalWallet {
    pub fn new(private_key: &str) -> Result<Self, Error> {
        let private_key = private_key.parse::<PrivateKey>()?;
        let address = ZilAddress::try_from(&private_key.public_key())?;

        Ok(Self {
            address,
            public_key: private_key.public_key(),
            private_key,
        })
    }

    pub fn create_random() -> Result<Self, Error> {
        let private_key = generate_private_key();
        Self::new(&private_key)
    }
}

impl Signer for LocalWallet {
    fn sign(&self, message: &[u8]) -> Signature {
        sign(message, &self.private_key)
    }

    fn address(&self) -> &ZilAddress {
        &self.address
    }

    fn public_key(&self) -> &PublicKey {
        &self.public_key
    }
}

impl FromStr for LocalWallet {
    type Err = Error;

    fn from_str(private_key: &str) -> Result<Self, Self::Err> {
        Self::new(private_key)
    }
}

#[cfg(test)]
mod tests {
    use claim::assert_some;

    use crate::{
        crypto::{schnorr::verify, ZilAddress},
        signers::Signer,
    };

    use super::LocalWallet;

    #[test]
    fn a_valid_private_key_should_results_a_valid_account_with_parse_function() {
        let account: LocalWallet = "0xD96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba"
            .parse()
            .unwrap();
        assert_eq!(
            account.address(),
            &"0x381f4008505e940AD7681EC3468a719060caF796".parse::<ZilAddress>().unwrap()
        );
    }

    #[test]
    fn a_valid_private_key_should_results_a_valid_account_with_new() {
        let account = LocalWallet::new("0xD96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba").unwrap();
        assert_eq!(
            account.address(),
            &"0x381f4008505e940AD7681EC3468a719060caF796".parse::<ZilAddress>().unwrap()
        );
    }

    #[test]
    fn sign_should_return_signature() {
        let account = LocalWallet::new("0xD96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba").unwrap();

        let signature = account.sign(&hex::decode("11223344aabb").unwrap());
        println!("{} {}", signature.r().to_string(), signature.s().to_string());

        assert_some!(verify(
            &hex::decode("11223344aabb").unwrap(),
            &account.public_key(),
            &signature
        ));
    }
}
