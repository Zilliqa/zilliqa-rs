pub mod local_wallet;

pub use local_wallet::LocalWallet;

use k256::ecdsa::Signature;

use crate::{
    crypto::{PublicKey, ZilAddress},
    providers::CreateTransactionRequest,
};

pub trait Signer {
    fn sign(&self, message: &[u8]) -> Signature;
    fn sign_transaction(&self, tx: &CreateTransactionRequest) -> Signature;
    fn public_key(&self) -> &PublicKey;
    fn address(&self) -> &ZilAddress;
}
