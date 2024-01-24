use std::{path::Path, str::FromStr};

use eth_keystore::{decrypt_key, encrypt_key};
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

    /// Loads a wallet from a keystore file.
    ///
    /// # Example
    /// ```
    /// use zilliqa_rs::signers::LocalWallet;
    /// use std::path::Path;
    /// let wallet = LocalWallet::load_keystore(&Path::new("./tests/keystore.json"), "zxcvbnm,").unwrap();
    /// ```
    pub fn load_keystore(path: &Path, password: &str) -> Result<Self, Error> {
        PrivateKey::from_slice(&decrypt_key(path, password).unwrap())?.try_into()
    }

    /// Encrypts the given wallet using the Scrypt password-based key derivation function, and stores it in the provided path. On success, it returns the id (Uuid) generated for this keystore.
    ///
    /// # Example
    /// ```
    /// use zilliqa_rs::signers::LocalWallet;
    /// use std::path::Path;
    /// use std::env;
    ///
    /// let wallet = LocalWallet::create_random().unwrap();
    /// let path = env::temp_dir().join("test_keystore.json");
    /// let filename = wallet.save_keystore(&path, "zxcvbnm,").unwrap();
    /// ```
    pub fn save_keystore(&self, path: &Path, password: &str) -> Result<String, Error> {
        let mut rng = rand::thread_rng();
        if path.is_dir() {
            return Err(Error::IsADirectory);
        }

        let (dir, filename) = (
            path.parent().ok_or(Error::FailedToGetTheParentDirectory)?,
            path.file_name().map(|filename| filename.to_str().unwrap()),
        );
        Ok(encrypt_key(dir, &mut rng, self.private_key.to_bytes(), password, filename)?)
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
    use std::env;

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

    #[test]
    fn save_and_load_keystore_should_work_fine() {
        let wallet = LocalWallet::create_random().unwrap();
        let path = env::temp_dir().join("keystore.json");
        let password = "qwerty";
        wallet.save_keystore(&path, password).unwrap();

        let wallet2 = LocalWallet::load_keystore(&path, password).unwrap();
        assert_eq!(wallet.address, wallet2.address);
    }
}
