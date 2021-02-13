# Goals

1. use balances for GLMR transfers (DONE)

2. use EVM for ERC20 transfers (TODO)

3. x-transfer abstracts over both with CurrencyId (TODO)

Downward Message Passing

4. transfer from relay chain account to parachain account (TODO)

Upward Messaging Passing

5. transfer from parachain account to relay chain account (TODO)

XCMP is the last priority because Cumulus uses HRMP now, a stopgap solution.

## Cumulus

- **Horizontal Message Passing (HMP)** refers to mechanisms that are responsible for exchanging messages between parachains.
- **Vertical Message Passing (VMP)** is used for communication between the relay chain and parachains.

- **Downward Message Passing (DMP)** is a mechanism for delivering messages to parachains from the relay chain.
- **Upward Message Passing (UMP)** is a mechanism responsible for delivering messages in the opposite direction: from a parachain up to the relay chain. Upward messages are essentially byte blobs. However, they are interpreted by the relay-chain according to the XCM standard.

### Resources

- https://wiki.polkadot.network/docs/en/learn-crosschain
- https://research.web3.foundation/en/latest/polkadot/XCMP/index.html
- https://w3f.github.io/parachain-implementers-guide/messaging.html

## Acala

There is an `evm-bridge` pallet which uses the `module-evm` pallet (as `Config`'s associated type `EVM`) to make transfers and get balances/supply for ERC20 tokens.

In particular, it uses `InvokeContext` to make transfer calls in the EVM. They declare this object in `support`.

The `evm-bridge` pallet is used in the `module-currencies` pallet to execute an EVM-native transfer when the passed in `CurrencyId` is of the `ERC20` variant.

#### acala/evm vs frontier/evm

what functionality does Acala's evm pallet have that frontier/frame/pallet-evm does not have?

- scheduled calls using `pallet-scheduler`
- root_or_signed origin
- contract maintainers that enforce on-chain permissions for deploying updates

Some specific objects from Acala's evm:

```rust, ignore
#[derive(Clone, Eq, PartialEq, RuntimeDebug, Encode, Decode)]
pub struct ContractInfo {
    pub code_hash: H256,
    pub maintainer: EvmAddress,
    pub deployed: bool,
}

#[derive(Clone, Eq, PartialEq, RuntimeDebug, Encode, Decode)]
pub struct AccountInfo<T: Config> {
    pub nonce: T::Index,
    pub contract_info: Option<ContractInfo>,
    pub developer_deposit: Option<BalanceOf<T>>,
}
```

There is a storage item `Accounts` which maps the `EVMAddress` to `AccountInfo`. Every `H160` address (`EvmAddress`) stored in the map has ownership permissions associated with it to pay storage rent and update the contract.

```rust, ignore
/// Accounts info.
#[pallet::storage]
#[pallet::getter(fn accounts)]
pub type Accounts<T: Config> = StorageMap<_, Twox64Concat, EvmAddress, AccountInfo<T>>;
```

To deploy a contract, the following auxiliary function is called, which checks that the caller is the maintainer (when it is passed in; sometimes root calls this with `maintainer: None`).

```rust, ignore
/// Mark contract as deployed
///
/// If maintainer is provider then it will check maintainer
fn mark_deployed(contract: EvmAddress, maintainer: Option<EvmAddress>) -> DispatchResult {
    Accounts::<T>::mutate(contract, |maybe_account_info| -> DispatchResult {
        if let Some(AccountInfo {
            contract_info: Some(contract_info),
            ..
        }) = maybe_account_info.as_mut()
        {
            if let Some(maintainer) = maintainer {
                ensure!(contract_info.maintainer == maintainer, Error::<T>::NoPermission);
            }
            ensure!(!contract_info.deployed, Error::<T>::ContractAlreadyDeployed);
            contract_info.deployed = true;
            Ok(())
        } else {
            Err(Error::<T>::ContractNotFound.into())
        }
    })
}
```

#### AddressMapping

Because Acala does not use H160 as the native `AccountId` type, it cannot use a trivial `AddressMapping` type like we do in Moonbeam (see `pallet_evm::IdentityAddressMapping`).

For this reason, they have a mechanism wherein substrate accounts claim ownership of Ethereum addresses. The `AddressMapping` associated type for the Acala `evm::Config` is constrained by the following trait (found in `Acala/primitives/evm`).

```rust, ignore
/// A mapping between `AccountId` and `EvmAddress`.
pub trait AddressMapping<AccountId> {
	fn get_account_id(evm: &EvmAddress) -> AccountId;
	fn get_evm_address(account_id: &AccountId) -> Option<EvmAddress>;
	fn get_or_create_evm_address(account_id: &AccountId) -> EvmAddress;
	fn get_default_evm_address(account_id: &AccountId) -> EvmAddress;
	fn is_linked(account_id: &AccountId, evm: &EvmAddress) -> bool;
}
```
