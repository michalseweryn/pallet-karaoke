#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_none;
use frame_support::inherent::{ProvideInherent, InherentData, IsFatalError};
use frame_support::pallet_prelude::InherentIdentifier;
use codec::WrapperTypeEncode;
use std::ops::Deref;

type InherentType = u8;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
	trait Store for Module<T: Config> as KaraokeModule {
		LineIndex get(fn line_index): Option<u8>;
	}
}

// Pallets use events to inform users when important changes are made.
decl_event!(
	pub enum Event<T> where <T as frame_system::Config>::AccountId {
		/// Line index was set. [index]
		LineIndexSet(u8, Option<AccountId>),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Config> { }
}

impl<T: Config> IsFatalError for Error<T> {
	fn is_fatal_error(&self) -> bool {
		true
	}
}

impl<T: Config> Deref for Error<T> {
	type Target = ();

	fn deref(&self) -> &Self::Target {
		&()
	}
}

impl<T: Config> WrapperTypeEncode for Error<T> {

}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn set_line_index(origin, line_index: u8) -> dispatch::DispatchResult {
			ensure_none(origin)?;

			LineIndex::put(line_index);

			Self::deposit_event(RawEvent::LineIndexSet(line_index, None));
			Ok(())
		}
	}
}

impl<T: Config> ProvideInherent for Module<T> {
	type Call = Call<T>;
	type Error = Error<T>;
	const INHERENT_IDENTIFIER: InherentIdentifier = *b"karaoke0";

	fn create_inherent(data: &InherentData) -> Option<Self::Call> {
		let line_index =
			data.get_data::<InherentType>(&Self::INHERENT_IDENTIFIER).ok()??;
		Some(Call::set_line_index(line_index))
	}
}