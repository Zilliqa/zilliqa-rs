use async_trait::async_trait;

use crate::{account::Transaction, providers::CreateTransactionResponse, signers::LocalWallet};

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
        // TODO: Refactor version
        if tx.version == 0 {
            let msg_version = 1u32;
            let chain_id = self.inner().get_chainid();
            let version = (chain_id as u32) << 16 | msg_version;
            tx.version = version;
        }

        let signature = self.sign_transaction(&tx)?;
        tx.signature = Some(hex::encode(signature.to_bytes()));

        tx.pub_key = Some(self.signer.public_key.clone());

        println!("{:?}", tx);
        self.inner().create_transaction(tx).await
    }

    fn sign_transaction(&self, tx: &Transaction) -> MiddlewareResult<crate::crypto::Signature> {
        self.signer.sign_transaction(tx).map_err(MiddlewareError::SignerError)
    }
}
