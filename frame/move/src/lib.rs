// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	decl_module, decl_storage, decl_event, decl_error, ensure,
};
use sp_std::prelude::*;
use frame_system::{ensure_signed, ensure_root};
use codec::{Encode, Decode};
use sp_runtime::AccountId32;
use omv::{
	primitives::{account_address::AccountAddress, language_storage::{ModuleId, StructTag, TypeTag}, identifier::Identifier, vm_status::StatusCode}, 
	serialize as bcs, 
	runtime::data_cache::RemoteCache,
	core::errors::{VMResult, PartialVMResult, PartialVMError, Location},
};

pub trait Config: frame_system::Config<AccountId=AccountId32> {
	type Event: From<Event> + Into<<Self as frame_system::Config>::Event>;
}

pub type RawIdentifier = Vec<u8>;
pub type RawStructTag = Vec<u8>;

decl_storage! {
	trait Store for Module<T: Config> as Move {
		Modules: double_map hasher(blake2_128_concat) AccountId32, hasher(blake2_128_concat) RawIdentifier => Option<Vec<u8>>;
		Resources: double_map hasher(blake2_128_concat) AccountId32, hasher(blake2_128_concat) RawStructTag => Option<Vec<u8>>;
	}
}

decl_error! {
	pub enum Error for Module<T: Config> {
		InvalidModuleIdentifier,
	}
}

decl_event!(
	pub enum Event {
		Dummy,
	}
);

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		#[weight = 0]
		fn publish(origin, identifier_raw: Vec<u8>, module_data: Vec<u8>) {
			let account_id = ensure_signed(origin)?;
			let identifier = Identifier::from_utf8(identifier_raw.clone()).map_err(|_| Error::<T>::InvalidModuleIdentifier)?;
			ensure!(Identifier::is_valid(identifier.as_str()), Error::<T>::InvalidModuleIdentifier);

			// TODO: reject backward-incompatible publishing.
			
			Modules::insert(account_id, identifier_raw, module_data);
		}
	}
}

impl<T: Config> Module<T> {

}

impl<T: Config> omv::runtime::data_cache::RemoteCache for Module<T> {
    fn get_module(&self, module_id: &ModuleId) -> VMResult<Option<Vec<u8>>> {
		let address = AccountId32::new(module_id.address().to_u8());
		let identifier = module_id.name().to_owned().into_bytes();

		Ok(Modules::get(&address, &identifier))
    }

    fn get_resource(
        &self,
        address: &AccountAddress,
        struct_tag: &StructTag,
    ) -> PartialVMResult<Option<Vec<u8>>> {
		let address = AccountId32::new(address.to_u8());
		let tag = &omv::serialize::to_bytes(struct_tag).map_err(|_| PartialVMError::new(StatusCode::STORAGE_ERROR))?;

		Ok(Resources::get(&address, &tag))
    }
}