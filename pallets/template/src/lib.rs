#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch,
    traits::Get, traits::Vec, sp_std::convert::TryInto};
use frame_system::{ensure_signed, ensure_root};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
	// Maps key id to the enabled/disabled state
	//type AuthorizedKeys: StoredMap<u64, bool>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
    trait Store for Module<T: Trait> as TemplateModule {
        MaxTransactionSize get(fn max_transaction_size): u32 = 65536; // 64kb
        LastTransactionId get(fn last_transaction_id): u64 = 0;
		AuthorizedKeys get(fn authorized_keys): map hasher(identity) u32 => bool;
    }
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
    pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
        Transaction(AccountId, u64, u32, u32, Vec<u8>), // [ account_id, tx_id, version, key_id, data ]
    }
);

// Errors inform users that something went wrong.
decl_error! {
    pub enum Error for Module<T: Trait> {
        InvalidTransactionVersion, // unsupported transaction version
        EmptyTransaction, // data is 0
        TransactionOverflow, // data exceeds max transaction size
		InvalidKeyId, // Key is disabled or not authorized
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;

        // Add a datalog transaction to the database.
        #[weight = 10_000 + T::DbWeight::get().reads_writes(3, 2)]
        pub fn transact(origin, version: u32, key_id: u32, data: Vec<u8>) -> dispatch::DispatchResult {
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
                        let valid_key_id;
                        if key_id == 0 {
                            if AuthorizedKeys::contains_key(key_id) {
                                valid_key_id = AuthorizedKeys::get(key_id);
                            }
                            else {
                                // key id 0 is the null key (unencrypted data)
                                // default to unencrypted
                                valid_key_id = true;
                            }
                        }
                        else {
                            valid_key_id = AuthorizedKeys::get(key_id);
                        }
						if valid_key_id {
							let tx_id = LastTransactionId::get() + 1;
							LastTransactionId::set(tx_id);
							Self::deposit_event(RawEvent::Transaction(who, tx_id, version, key_id, data));
							Ok(())
						}
                        else {
                            Err(Error::<T>::InvalidKeyId)?
                        }
                    },
                    _ => Err(Error::<T>::InvalidTransactionVersion)?,
                }
            }
        }

        // Change the max transaction size
        #[weight = 10_000 + T::DbWeight::get().reads_writes(0, 1)]
        pub fn admin_set_max_transaction_size(origin, size: u32) -> dispatch::DispatchResult {
            ensure_root(origin)?;
            MaxTransactionSize::set(size);
            Ok(())
        }

		// Add an authorized key id
		#[weight = 10_000 + T::DbWeight::get().reads_writes(0, 1)]
		pub fn admin_enable_key(origin, key_id: u32) -> dispatch::DispatchResult {
            ensure_root(origin)?;
			AuthorizedKeys::insert(key_id, true);
			Ok(())
		}

		// Remove an authorized key id
		#[weight = 10_000 + T::DbWeight::get().reads_writes(0, 1)]
		pub fn admin_disable_key(origin, key_id: u32) -> dispatch::DispatchResult {
            ensure_root(origin)?;
			AuthorizedKeys::insert(key_id, false);
			Ok(())
		}
    }
}
