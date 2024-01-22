# Changelog

## [[0.2.0]](https://github.com/Zilliqa/zilliqa-rs/releases/tag/v0.2.0) - 2023-01-18

### Added
- Additional documentation.

## [[0.1.0]](https://github.com/Zilliqa/zilliqa-rs/releases/tag/v0.1.0) - 2023-01-10

### Added
- Providers are supported in general.
- HTTP provider is implemented.
- All JSON-RPC endpoints are supported.
- Signer, something that can sign is implemented in general.
- LocalWallet is implemented as a signer.
- MultAccountWallet is implemented as a signer.
- Signing and sending payment transactions are supported.
- [TransactionBuilder ](https://github.com/Zilliqa/zilliqa-rs/blob/1547779629aa09025047625b410225752b393302/tests/send_transaction.rs#L27)is added which simplifies the process of composing transactions.
- Contract deployment transactions are supported.
- Interacting with contracts is implemented.
- Rust binding for scilla contracts is implemented. Corresponding rust code is generated for scilla contracts at compile time.
  - It has a deploy function. It's not possible to deploy a contract with the wrong parameters accidentally.
  - It has all of the transitions as rust functions. [Parameters to transitions are type-safe](https://github.com/Zilliqa/zilliqa-rs/blob/master/tests/call_transition.rs).
  - It has some functions to get the current [state of the contract](https://github.com/Zilliqa/zilliqa-rs/blob/master/tests/deploy_contract.rs#L172).
  - All scilla types are converted to rust ones. Even complex ones [like maps and lists](https://github.com/Zilliqa/zilliqa-rs/blob/master/tests/get_field.rs).
- parse_zil and a few more auxiliary functions are added to simplify the [unit conversion](https://github.com/Zilliqa/zilliqa-rs/blob/master/tests/send_transaction.rs#L110).
- Ethers-rs-like middleware system is supported.
- All of the data types are type-safe, you can't pass a raw string when a real address is needed.
- Possible to call a transition [with a different signer](https://github.com/Zilliqa/zilliqa-rs/blob/1547779629aa09025047625b410225752b393302/tests/deploy_contract.rs#L193) than the one who deployed the contract.
- Possible to override [default transaction parameters](https://github.com/Zilliqa/zilliqa-rs/blob/1547779629aa09025047625b410225752b393302/tests/acceptance_tests/send_zil.rs#L16) such as amount and gasPrice, etc while calling a transition.
