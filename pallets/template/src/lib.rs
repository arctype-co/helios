#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch,
    traits::Get, traits::Vec, sp_std::convert::TryInto};
use frame_system::ensure_signed;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
    trait Store for Module<T: Trait> as TemplateModule {
        MaxTransactionSize get(fn size): u32 = 65536; // 64kb
    }
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
    pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
        Transaction(AccountId, u32, Vec<u8>), // [ accountId, version, data ]
    }
);

// Errors inform users that something went wrong.
decl_error! {
    pub enum Error for Module<T: Trait> {
        InvalidTransactionVersion, // unsupported transaction version
        EmptyTransaction, // data is 0
        TransactionOverflow, // data exceeds max transaction size
    }
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;

        // Add a datalog transaction to the database.
        #[weight = 10_000 + T::DbWeight::get().reads_writes(1, 0)]
        pub fn transact(origin, version: u32, data: Vec<u8>) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            let max_size = MaxTransactionSize::get();
            let size = data.len();
            if size == 0 {
                Err(Error::<T>::EmptyTransaction)?
            }
            else if size > max_size.try_into().unwrap() {
                Err(Error::<T>::TransactionOverflow)?
            }
            else {
                match version {
                    0 => {
                        Self::deposit_event(RawEvent::Transaction(who, version, data));
                        Ok(())
                    },
                    _ => Err(Error::<T>::InvalidTransactionVersion)?,
                }
            }
        }

        // Change the max transaction size
        #[weight = 10_000 + T::DbWeight::get().reads_writes(0, 1)]
        pub fn admin_set_max_transaction_size(origin, size: u32) -> dispatch::DispatchResult {
            // TODO ensure origin has sudo
            MaxTransactionSize::set(size);
            Ok(())
        }
    }
}
