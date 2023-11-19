use k256::{ecdsa::Signature, PublicKey, SecretKey};
use primitive_types::H160;
use prost::Message;

use crate::{
    crypto::{
        bech32::to_bech32_address,
        schnorr::sign,
        util::{get_address_from_public_key, get_pub_key_from_private_key, normalize_private_key},
    },
    proto::{Nonce, ProtoTransactionCoreInfo},
};

use super::{error::AccountError, Transaction};

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

    pub fn sign(&self, message: &[u8]) -> Signature {
        let secret_key = SecretKey::from_slice(&hex::decode(&self.private_key).unwrap()).unwrap();
        sign(message, &secret_key)
    }

    pub fn sign_transaction(&self, mut tx: Transaction) -> Transaction {
        let to_addr: H160 = tx.to_addr.parse().unwrap();
        let public_key =
            PublicKey::from_sec1_bytes(&hex::decode(&self.public_key).unwrap()).unwrap();
        let secret_key = SecretKey::from_slice(&hex::decode(&self.private_key).unwrap()).unwrap();

        let proto = ProtoTransactionCoreInfo {
            version: tx.version,
            toaddr: to_addr.as_bytes().to_vec(),
            senderpubkey: Some(public_key.to_sec1_bytes().into()),
            amount: Some(tx.amount.to_be_bytes().to_vec().into()),
            gasprice: Some(tx.gas_price.to_be_bytes().to_vec().into()),
            gaslimit: tx.gas_limit,
            oneof2: Some(Nonce::Nonce(tx.nonce)),
            oneof8: None,
            oneof9: None,
        };

        let txn_data = proto.encode_to_vec();
        let signature = sign(&txn_data, &secret_key);

        tx.signature = Some(hex::encode(signature.to_bytes()));
        tx.pub_key = Some(self.public_key.clone());
        tx
    }
}

#[cfg(test)]
mod tests {
    use k256::PublicKey;

    use crate::crypto::schnorr::verify;

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

    #[test]
    fn sign_should_return_signature() {
        let account =
            Account::new("0xD96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba")
                .unwrap();

        let signature = account.sign(&hex::decode("11223344aabb").unwrap());
        println!(
            "{} {}",
            signature.r().to_string(),
            signature.s().to_string()
        );

        let public_key =
            PublicKey::from_sec1_bytes(&hex::decode(account.public_key).unwrap()).unwrap();

        let v = verify(&hex::decode("11223344aabb").unwrap(), public_key, signature);
        println!("{:?}", v);
    }
}
