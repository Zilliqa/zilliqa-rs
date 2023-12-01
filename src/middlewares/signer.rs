use async_trait::async_trait;
use serde::de::DeserializeOwned;

use crate::{providers::CreateTransactionRequest, signers::LocalWallet, transaction::Version, Error};

use super::Middleware;

#[derive(Debug)]
pub struct SignerMiddleware<M> {
    // TODO: Make this generic
    signer: LocalWallet,
    inner: M,
}

impl<M: Middleware> SignerMiddleware<M> {
    pub fn new(inner: M, signer: LocalWallet) -> Self {
        Self { signer, inner }
    }
}

#[async_trait]
impl<M: Middleware> Middleware for SignerMiddleware<M> {
    type Provider = M::Provider;

    type Inner = M;

    fn inner(&self) -> &Self::Inner {
        &self.inner
    }

    fn is_signer(&self) -> bool {
        true
    }

    async fn send_transaction_without_confirm<T: Send + DeserializeOwned>(
        &self,
        mut tx: CreateTransactionRequest,
    ) -> Result<T, Error> {
        if !tx.version.valid() {
            tx.version = Version::new(self.inner().get_chainid());
        }

        // TODO: Make it a middleware like ethers-rs
        // TODO: Is it a sane condition?
        if tx.nonce == u64::default() {
            let balance = self.inner().get_balance(&self.signer.address).await?;
            tx.nonce = balance.nonce + 1;
        }

        let signature = self.sign_transaction(&tx)?;
        tx.signature = Some(hex::encode(signature.to_bytes()));

        tx.pub_key = Some(self.signer.public_key().to_string().clone());

        self.inner().create_transaction(tx).await
    }

    fn sign_transaction(&self, tx: &CreateTransactionRequest) -> Result<crate::crypto::Signature, Error> {
        self.signer.sign_transaction(tx)
    }

    fn sign(&self, data: &[u8]) -> Result<crate::crypto::Signature, Error> {
        Ok(self.signer.sign(data))
    }
}
