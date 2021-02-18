// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	decl_module, decl_storage, decl_event,
};
use sp_std::prelude::*;
use frame_system::{ensure_signed, ensure_root};
use codec::{Encode, Decode};
use omv::{
	primitives::{account_address::AccountAddress, language_storage::{ModuleId, StructTag, TypeTag}, vm_status::StatusCode}, 
	serialize as bcs, 
	runtime::data_cache::RemoteCache,
	core::errors::{VMResult, PartialVMResult, PartialVMError, Location},
};

pub trait Config: frame_system::Config {
	type Event: From<Event> + Into<<Self as frame_system::Config>::Event>;
}

pub type RawModuleId = Vec<u8>;
pub type RawAccountAddress = Vec<u8>;
pub type RawStructTag = Vec<u8>;

decl_storage! {
	trait Store for Module<T: Config> as Move {
		Modules: map hasher(blake2_128_concat) RawModuleId => Option<Vec<u8>>;
		Resources: double_map hasher(blake2_128_concat) RawAccountAddress, hasher(blake2_128_concat) RawStructTag => Option<Vec<u8>>;
	}
}

decl_event!(
	pub enum Event {
		Dummy,
	}
);

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		fn deposit_event() = default;
	}
}

impl<T: Config> Module<T> {

}

impl<T: Config> omv::runtime::data_cache::RemoteCache for Module<T> {
    fn get_module(&self, module_id: &ModuleId) -> VMResult<Option<Vec<u8>>> {
		Ok(Modules::get(&omv::serialize::to_bytes(module_id).map_err(|_| PartialVMError::new(StatusCode::STORAGE_ERROR).finish(Location::Undefined).into())?))
    }

    fn get_resource(
        &self,
        address: &AccountAddress,
        struct_tag: &StructTag,
    ) -> PartialVMResult<Option<Vec<u8>>> {
		Ok(Resources::get(
			&omv::serialize::to_bytes(address).map_err(|_| PartialVMError::new(StatusCode::STORAGE_ERROR))?,
			&omv::serialize::to_bytes(struct_tag).map_err(|_| PartialVMError::new(StatusCode::STORAGE_ERROR))?,
		))
    }
}