use std::str::FromStr;

use k256::ecdsa::Signature;

use crate::{
    core::{PrivateKey, PublicKey, ZilAddress},
    crypto::schnorr::sign,
    Error,
};

use super::Signer;

/// Represents a local wallet, containing a private key, address, and public key.
#[derive(Debug, Clone, PartialEq)]
pub struct LocalWallet {
    /// Private key of the wallet.
    pub private_key: PrivateKey,
    /// Public address of the wallet which is used to receive ZIL.
    pub address: ZilAddress,
    /// Public key of the wallet.
    public_key: PublicKey,
}

impl LocalWallet {
    /// Generates a random private key and creates a new instance of a LocalWallet
    /// using that key.
    ///
    /// # Example
    /// ```
    /// use zilliqa_rs::signers::LocalWallet;
    /// let wallet = LocalWallet::create_random().unwrap();
    /// ```
    pub fn create_random() -> Result<Self, Error> {
        PrivateKey::create_random().try_into()
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

    /// Create a new LocalWallet out of a string slice containing a private key.
    fn from_str(private_key: &str) -> Result<Self, Self::Err> {
        private_key.parse::<PrivateKey>()?.try_into()
    }
}

impl TryFrom<PrivateKey> for LocalWallet {
    type Error = Error;

    /// Converts a private key to a LocalWallet.
    ///
    /// # Example
    /// ```
    /// use zilliqa_rs::signers::LocalWallet;
    /// use zilliqa_rs::core::PrivateKey;
    ///
    /// let private_key = PrivateKey::create_random();
    /// let wallet = LocalWallet::try_from(private_key).unwrap();
    /// ```
    fn try_from(private_key: PrivateKey) -> Result<Self, Error> {
        let address = ZilAddress::try_from(&private_key.public_key())?;

        Ok(Self {
            address,
            public_key: private_key.public_key(),
            private_key,
        })
    }
}

#[cfg(test)]
mod tests {
    use claim::assert_some;

    use crate::{core::ZilAddress, crypto::schnorr::verify, signers::Signer};

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
        let account: LocalWallet = "0xD96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba"
            .parse()
            .unwrap();
        assert_eq!(
            account.address(),
            &"0x381f4008505e940AD7681EC3468a719060caF796".parse::<ZilAddress>().unwrap()
        );
    }

    #[test]
    fn sign_should_return_signature() {
        let account: LocalWallet = "0xD96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba"
            .parse()
            .unwrap();

        let signature = account.sign(&hex::decode("11223344aabb").unwrap());
        println!("{} {}", signature.r().to_string(), signature.s().to_string());

        assert_some!(verify(
            &hex::decode("11223344aabb").unwrap(),
            &account.public_key(),
            &signature
        ));
    }
}
