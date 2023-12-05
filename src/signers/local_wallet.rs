use std::str::FromStr;

use k256::ecdsa::Signature;
use primitive_types::H160;
use prost::Message;

use crate::{
    crypto::{generate_private_key, schnorr::sign, PrivateKey, PublicKey, ZilAddress},
    proto::{Nonce, ProtoTransactionCoreInfo},
    providers::CreateTransactionRequest,
    Error,
};

#[derive(Debug, Clone, PartialEq)]
pub struct LocalWallet {
    pub private_key: PrivateKey,
    pub address: ZilAddress,
}

impl LocalWallet {
    pub fn new(private_key: &str) -> Result<Self, Error> {
        let private_key = private_key.parse::<PrivateKey>()?;
        let address = ZilAddress::try_from(&private_key.public_key())?;

        Ok(Self { private_key, address })
    }

    pub fn create_random() -> Result<Self, Error> {
        let private_key = generate_private_key();
        Self::new(&private_key)
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        sign(message, &self.private_key)
    }

    pub fn public_key(&self) -> PublicKey {
        self.private_key.public_key()
    }

    pub fn sign_transaction(&self, tx: &CreateTransactionRequest) -> Result<Signature, Error> {
        let to_addr: H160 = tx.to_addr.parse().unwrap();

        let mut amount = [0_u8; 32];
        tx.amount.to_big_endian(&mut amount);

        let mut gas_price = [0_u8; 32];
        tx.gas_price.to_big_endian(&mut gas_price);

        let proto = ProtoTransactionCoreInfo {
            version: tx.version.pack(),
            toaddr: to_addr.as_bytes().to_vec(),
            senderpubkey: Some(self.private_key.public_key().to_sec1_bytes().into()),
            amount: Some(amount.to_vec().into()),
            gasprice: Some(gas_price.to_vec().into()),
            gaslimit: tx.gas_limit,
            oneof2: Some(Nonce::Nonce(tx.nonce)),
            //TODO: Remove clones
            oneof8: tx.code.clone().map(|code| crate::proto::Code::Code(code.as_bytes().to_vec())),
            oneof9: tx.data.clone().map(|data| crate::proto::Data::Data(data.as_bytes().to_vec())),
        };

        let txn_data = proto.encode_to_vec();
        Ok(sign(&txn_data, &self.private_key))
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

    use crate::crypto::schnorr::verify;

    use super::LocalWallet;

    #[test]
    fn a_valid_private_key_should_results_a_valid_account_with_parse_function() {
        let account: LocalWallet = "0xD96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba"
            .parse()
            .unwrap();
        assert_eq!(
            account,
            LocalWallet {
                private_key: "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba"
                    .parse()
                    .unwrap(),
                address: "0x381f4008505e940AD7681EC3468a719060caF796".parse().unwrap()
            }
        );
    }

    #[test]
    fn a_valid_private_key_should_results_a_valid_account_with_new() {
        let account = LocalWallet::new("0xD96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba").unwrap();
        assert_eq!(
            account,
            LocalWallet {
                private_key: "d96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba"
                    .parse()
                    .unwrap(),
                address: "0x381f4008505e940AD7681EC3468a719060caF796".parse().unwrap()
            }
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
