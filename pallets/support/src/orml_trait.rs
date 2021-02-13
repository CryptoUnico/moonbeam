// Copyright 2019-2020 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

//! Clone of ORML-trait objects used in Moonbeam Runtime so we don't have to sync dependency updates
//! [orml](https://github.com/open-web3-stack/open-runtime-module-library)
use sp_runtime::{
	traits::{AtLeast32BitUnsigned, MaybeSerializeDeserialize}, DispatchResult,
};
use parity_scale_codec::FullCodec;
use sp_std::fmt::Debug;

/// Abstraction over a fungible multi-currency system.
pub trait MultiCurrency<AccountId> {
	/// The currency identifier.
	type CurrencyId: FullCodec + Eq + PartialEq + Copy + MaybeSerializeDeserialize + Debug;

	/// The balance of an account.
	type Balance: AtLeast32BitUnsigned + FullCodec + Copy + MaybeSerializeDeserialize + Debug + Default;

	// Public immutables

	/// Existential deposit of `currency_id`.
	fn minimum_balance(currency_id: Self::CurrencyId) -> Self::Balance;

	/// The total amount of issuance of `currency_id`.
	fn total_issuance(currency_id: Self::CurrencyId) -> Self::Balance;

	// The combined balance of `who` under `currency_id`.
	fn total_balance(currency_id: Self::CurrencyId, who: &AccountId) -> Self::Balance;

	// The free balance of `who` under `currency_id`.
	fn free_balance(currency_id: Self::CurrencyId, who: &AccountId) -> Self::Balance;

	/// A dry-run of `withdraw`. Returns `Ok` iff the account is able to make a
	/// withdrawal of the given amount.
	fn ensure_can_withdraw(currency_id: Self::CurrencyId, who: &AccountId, amount: Self::Balance) -> DispatchResult;

	// Public mutables

	/// Transfer some amount from one account to another.
	fn transfer(
		currency_id: Self::CurrencyId,
		from: &AccountId,
		to: &AccountId,
		amount: Self::Balance,
	) -> DispatchResult;

	/// Add `amount` to the balance of `who` under `currency_id` and increase
	/// total issuance.
	fn deposit(currency_id: Self::CurrencyId, who: &AccountId, amount: Self::Balance) -> DispatchResult;

	/// Remove `amount` from the balance of `who` under `currency_id` and reduce
	/// total issuance.
	fn withdraw(currency_id: Self::CurrencyId, who: &AccountId, amount: Self::Balance) -> DispatchResult;

	/// Same result as `slash(currency_id, who, value)` (but without the
	/// side-effects) assuming there are no balance changes in the meantime and
	/// only the reserved balance is not taken into account.
	fn can_slash(currency_id: Self::CurrencyId, who: &AccountId, value: Self::Balance) -> bool;

	/// Deduct the balance of `who` by up to `amount`.
	///
	/// As much funds up to `amount` will be deducted as possible.  If this is
	/// less than `amount`,then a non-zero value will be returned.
	fn slash(currency_id: Self::CurrencyId, who: &AccountId, amount: Self::Balance) -> Self::Balance;
}