//! A unified interface for locally signing zilliqa transactions.

pub mod local_wallet;
pub mod multi_account_wallet;

pub use local_wallet::LocalWallet;

use k256::ecdsa::Signature;

use crate::core::CreateTransactionRequest;
use crate::crypto::{PublicKey, ZilAddress};

pub trait Signer {
    fn sign(&self, message: &[u8]) -> Signature;
    fn sign_transaction(&self, tx: &CreateTransactionRequest) -> Signature {
        self.sign(&tx.proto_encode(self.public_key().to_sec1_bytes().into()))
    }

    fn public_key(&self) -> &PublicKey;
    fn address(&self) -> &ZilAddress;
}
