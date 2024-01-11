use std::{collections::HashMap, rc::Rc};

use crate::{crypto::ZilAddress, Error};

use super::{LocalWallet, Signer};

#[derive(Default)]
pub struct MultiAccountWallet {
    default_account: Option<ZilAddress>,
    accounts: HashMap<ZilAddress, Rc<LocalWallet>>,
}

impl MultiAccountWallet {
    pub fn new_with_accounts(accounts: Vec<LocalWallet>) -> Self {
        let default_account = if !accounts.is_empty() {
            Some(accounts[0].address.clone())
        } else {
            None
        };

        let accounts = accounts
            .into_iter()
            .map(|account| (account.address.clone(), Rc::new(account)))
            .collect::<HashMap<_, _>>();

        Self {
            accounts,
            default_account,
        }
    }

    pub fn create(&mut self) -> Result<Rc<LocalWallet>, Error> {
        let wallet = Rc::new(LocalWallet::create_random()?);
        self.add_local_wallet(wallet.clone());
        Ok(wallet.clone())
    }

    pub fn add_local_wallet(&mut self, wallet: Rc<LocalWallet>) -> Rc<LocalWallet> {
        if self.default_account.is_none() {
            self.default_account = Some(wallet.address.clone())
        }
        self.accounts.insert(wallet.address.clone(), wallet.clone());
        wallet
    }

    pub fn add_by_private_key(&mut self, private_key: &str) -> Result<Rc<LocalWallet>, Error> {
        let wallet = private_key.parse::<LocalWallet>()?;
        Ok(self.add_local_wallet(Rc::new(wallet)))
    }

    pub fn remove(&mut self, address: &ZilAddress) -> Option<Rc<LocalWallet>> {
        if let Some(account) = &self.default_account {
            if account == address {
                self.default_account = None;
            }
        }
        self.accounts.remove(address)
    }

    pub fn set_default(&mut self, address: &ZilAddress) -> Result<Rc<LocalWallet>, Error> {
        let account = self
            .accounts
            .get(address)
            .ok_or(Error::AccountDoesNotExist(address.to_string()))?;

        self.default_account = Some(address.clone());

        Ok(account.clone())
    }

    pub fn default_account(&self) -> Option<Rc<LocalWallet>> {
        if let Some(address) = &self.default_account {
            self.accounts.get(address).cloned()
        } else {
            None
        }
    }
}

impl Signer for MultiAccountWallet {
    fn sign(&self, message: &[u8]) -> k256::ecdsa::Signature {
        self.default_account().unwrap().sign(message)
    }

    // FIXME:
    fn public_key(&self) -> &crate::crypto::PublicKey {
        todo!()
    }

    fn address(&self) -> &ZilAddress {
        self.default_account.as_ref().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::MultiAccountWallet;
    use claim::assert_none;

    #[test]
    fn wallet_create_function_should_create_a_new_account() {
        let mut multi_account_wallet = MultiAccountWallet::default();
        let wallet = multi_account_wallet.create().unwrap();
        assert_eq!(multi_account_wallet.accounts.len(), 1);

        let account = multi_account_wallet.accounts.get(&wallet.address).unwrap();
        assert_eq!(wallet.address, account.address);

        assert!(multi_account_wallet.default_account.is_some());
        assert_eq!(multi_account_wallet.default_account.unwrap(), account.address);
    }

    #[test]
    fn add_by_private_key_function_should_create_a_new_account_in_wallet() {
        let mut wallet = MultiAccountWallet::default();
        let private_key = "0xD96e9eb5b782a80ea153c937fa83e5948485fbfc8b7e7c069d7b914dbc350aba";

        let local_wallet = wallet.add_by_private_key(&private_key).unwrap();
        assert_eq!(wallet.accounts.len(), 1);

        let account = wallet.accounts.get(&local_wallet.address).unwrap();

        assert!(wallet.default_account.is_some());
        assert_eq!(wallet.default_account.unwrap(), account.address);
    }

    #[test]
    fn remove_should_return_non_if_address_does_not_exist_in_wallet() {
        let mut wallet = MultiAccountWallet::default();
        wallet.create().unwrap();

        assert_none!(wallet.remove(&"0x381f4008505e940AD7681EC3468a719060caF796".parse().unwrap()));
    }

    #[test]
    fn remove_should_return_remove_account_from_wallet_if_address_exist() {
        let mut wallet = MultiAccountWallet::default();
        let local_wallet = wallet.create().unwrap();

        let removed_account = wallet.remove(&local_wallet.address).unwrap();
        assert_eq!(removed_account.address, local_wallet.address);
        assert_eq!(0, wallet.accounts.len());
        assert_none!(wallet.default_account); // Because we deleted the only available account in the wallet.
    }

    #[test]
    fn set_default_should_set_the_default_account_correctly() {
        let mut wallet = MultiAccountWallet::default();
        let local_wallet1 = wallet.create().unwrap();
        let local_wallet2 = wallet.create().unwrap();

        assert_eq!(wallet.default_account().unwrap().address, local_wallet1.address);

        wallet.set_default(&local_wallet2.address).unwrap();
        assert_eq!(wallet.default_account().unwrap().address, local_wallet2.address);
    }
}
