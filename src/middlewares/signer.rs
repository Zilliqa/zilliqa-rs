use async_trait::async_trait;

use crate::{
    providers::CreateTransactionResponse,
    signers::LocalWallet,
    transaction::{Transaction, Version},
};

use super::{Middleware, MiddlewareError, MiddlewareResult};

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

    async fn send_transaction(&self, mut tx: Transaction) -> MiddlewareResult<CreateTransactionResponse> {
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

        println!("{}", serde_json::to_string_pretty(&tx).unwrap());
        self.inner().create_transaction(tx).await
    }

    fn sign_transaction(&self, tx: &Transaction) -> MiddlewareResult<crate::crypto::Signature> {
        self.signer.sign_transaction(tx).map_err(MiddlewareError::SignerError)
    }
}
