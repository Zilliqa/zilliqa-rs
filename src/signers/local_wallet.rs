use std::str::FromStr;

use k256::ecdsa::Signature;

use crate::{
    core::{PrivateKey, PublicKey, ZilAddress},
    crypto::schnorr::sign,
    Error,
};

use super::Signer;

/// Represents a local wallet, containing a private key, address, and public key.
///
/// Properties:
///
/// * `private_key`: The `private_key` property is of type `PrivateKey` and represents the private key
/// of the wallet. It is used for signing transactions and proving ownership of the wallet.
/// * `address`: The `address` property is of type `ZilAddress` and represents the address of the
/// wallet. It is a public identifier that is used to receive funds or interact with the Zilliqa
/// blockchain.
/// * `public_key`: The `public_key` property is a public key associated with the `LocalWallet`. It is
/// used for cryptographic operations and can be shared with others.
#[derive(Debug, Clone, PartialEq)]
pub struct LocalWallet {
    pub private_key: PrivateKey,
    pub address: ZilAddress,
    public_key: PublicKey,
}

impl LocalWallet {
    /// The function takes a private key as input, parses it into a `PrivateKey` object, converts it into a
    /// `ZilAddress`, and returns a new instance of the struct containing the address, public key, and
    /// private key.
    ///
    /// Arguments:
    ///
    /// * `private_key`: A string representing the private key.
    pub fn new(private_key: PrivateKey) -> Result<Self, Error> {
        let address = ZilAddress::try_from(&private_key.public_key())?;

        Ok(Self {
            address,
            public_key: private_key.public_key(),
            private_key,
        })
    }

    /// The function `create_random` generates a random private key and creates a new instance of a struct
    /// using that key.
    pub fn create_random() -> Result<Self, Error> {
        let private_key = PrivateKey::generate();
        Self::new(private_key)
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

    /// This function create a new LocalWallet out of a string slice.
    /// Arguments:
    ///
    /// * `private_key`: A string representing the private key.
    fn from_str(private_key: &str) -> Result<Self, Self::Err> {
        let private_key: PrivateKey = private_key.parse()?;
        Self::new(private_key)
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
