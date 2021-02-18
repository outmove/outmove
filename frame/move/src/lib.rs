// This file is part of Substrate.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! <!-- markdown-link-check-disable -->
//! # Example Pallet
//!
//! <!-- Original author of paragraph: @gavofyork -->
//! The Example: A simple example of a FRAME pallet demonstrating
//! concepts, APIs and structures common to most FRAME runtimes.
//!
//! Run `cargo doc --package pallet-example --open` to view this pallet's documentation.
//!
//! ### Documentation Guidelines:
//!
//! <!-- Original author of paragraph: Various. Based on collation of review comments to PRs addressing issues with -->
//! <!-- label 'S3-FRAME' in https://github.com/paritytech/substrate-developer-hub/issues -->
//! <ul>
//!     <li>Documentation comments (i.e. <code>/// comment</code>) - should
//!         accompany pallet functions and be restricted to the pallet interface,
//!         not the internals of the pallet implementation. Only state inputs,
//!         outputs, and a brief description that mentions whether calling it
//!         requires root, but without repeating the source code details.
//!         Capitalize the first word of each documentation comment and end it with
//!         a full stop. See
//!         <a href="https://github.com/paritytech/substrate#72-contributing-to-documentation-for-substrate-packages"
//!         target="_blank"> Generic example of annotating source code with documentation comments</a></li>
//!     <li>Self-documenting code - Try to refactor code to be self-documenting.</li>
//!     <li>Code comments - Supplement complex code with a brief explanation, not every line of code.</li>
//!     <li>Identifiers - surround by backticks (i.e. <code>INHERENT_IDENTIFIER</code>, <code>InherentType</code>,
//!         <code>u64</code>)</li>
//!     <li>Usage scenarios - should be simple doctests. The compiler should ensure they stay valid.</li>
//!     <li>Extended tutorials - should be moved to external files and refer to.</li>
//!     <!-- Original author of paragraph: @AmarRSingh -->
//!     <li>Mandatory - include all of the sections/subsections where <b>MUST</b> is specified.</li>
//!     <li>Optional - optionally include sections/subsections where <b>CAN</b> is specified.</li>
//! </ul>
//!
//! ### Documentation Template:<br>
//!
//! Copy and paste this template from frame/example/src/lib.rs into file
//! `frame/<INSERT_CUSTOM_PALLET_NAME>/src/lib.rs` of your own custom pallet and complete it.
//! <details><p><pre>
//! // Add heading with custom pallet name
//!
//! \# <INSERT_CUSTOM_PALLET_NAME> Pallet
//!
//! // Add simple description
//!
//! // Include the following links that shows what trait needs to be implemented to use the pallet
//! // and the supported dispatchables that are documented in the Call enum.
//!
//! - \[`<INSERT_CUSTOM_PALLET_NAME>::Config`](./trait.Config.html)
//! - \[`Call`](./enum.Call.html)
//! - \[`Module`](./struct.Module.html)
//!
//! \## Overview
//!
//! <!-- Original author of paragraph: Various. See https://github.com/paritytech/substrate-developer-hub/issues/44 -->
//! // Short description of pallet's purpose.
//! // Links to Traits that should be implemented.
//! // What this pallet is for.
//! // What functionality the pallet provides.
//! // When to use the pallet (use case examples).
//! // How it is used.
//! // Inputs it uses and the source of each input.
//! // Outputs it produces.
//!
//! <!-- Original author of paragraph: @Kianenigma in PR https://github.com/paritytech/substrate/pull/1951 -->
//! <!-- and comment https://github.com/paritytech/substrate-developer-hub/issues/44#issuecomment-471982710 -->
//!
//! \## Terminology
//!
//! // Add terminology used in the custom pallet. Include concepts, storage items, or actions that you think
//! // deserve to be noted to give context to the rest of the documentation or pallet usage. The author needs to
//! // use some judgment about what is included. We don't want a list of every storage item nor types - the user
//! // can go to the code for that. For example, "transfer fee" is obvious and should not be included, but
//! // "free balance" and "reserved balance" should be noted to give context to the pallet.
//! // Please do not link to outside resources. The reference docs should be the ultimate source of truth.
//!
//! <!-- Original author of heading: @Kianenigma in PR https://github.com/paritytech/substrate/pull/1951 -->
//!
//! \## Goals
//!
//! // Add goals that the custom pallet is designed to achieve.
//!
//! <!-- Original author of heading: @Kianenigma in PR https://github.com/paritytech/substrate/pull/1951 -->
//!
//! \### Scenarios
//!
//! <!-- Original author of paragraph: @Kianenigma. Based on PR https://github.com/paritytech/substrate/pull/1951 -->
//!
//! \#### <INSERT_SCENARIO_NAME>
//!
//! // Describe requirements prior to interacting with the custom pallet.
//! // Describe the process of interacting with the custom pallet for this scenario and public API functions used.
//!
//! \## Interface
//!
//! \### Supported Origins
//!
//! // What origins are used and supported in this pallet (root, signed, none)
//! // i.e. root when <code>\`ensure_root\`</code> used
//! // i.e. none when <code>\`ensure_none\`</code> used
//! // i.e. signed when <code>\`ensure_signed\`</code> used
//!
//! <code>\`inherent\`</code> <INSERT_DESCRIPTION>
//!
//! <!-- Original author of paragraph: @Kianenigma in comment -->
//! <!-- https://github.com/paritytech/substrate-developer-hub/issues/44#issuecomment-471982710 -->
//!
//! \### Types
//!
//! // Type aliases. Include any associated types and where the user would typically define them.
//!
//! <code>\`ExampleType\`</code> <INSERT_DESCRIPTION>
//!
//! <!-- Original author of paragraph: ??? -->
//!
//! // Reference documentation of aspects such as `storageItems` and `dispatchable` functions should only be
//! // included in the <https://docs.rs> Rustdocs for Substrate and not repeated in the README file.
//!
//! \### Dispatchable Functions
//!
//! <!-- Original author of paragraph: @AmarRSingh & @joepetrowski -->
//!
//! // A brief description of dispatchable functions and a link to the rustdoc with their actual documentation.
//!
//! // <b>MUST</b> have link to Call enum
//! // <b>MUST</b> have origin information included in function doc
//! // <b>CAN</b> have more info up to the user
//!
//! \### Public Functions
//!
//! <!-- Original author of paragraph: @joepetrowski -->
//!
//! // A link to the rustdoc and any notes about usage in the pallet, not for specific functions.
//! // For example, in the Balances Pallet: "Note that when using the publicly exposed functions,
//! // you (the runtime developer) are responsible for implementing any necessary checks
//! // (e.g. that the sender is the signer) before calling a function that will affect storage."
//!
//! <!-- Original author of paragraph: @AmarRSingh -->
//!
//! // It is up to the writer of the respective pallet (with respect to how much information to provide).
//!
//! \#### Public Inspection functions - Immutable (getters)
//!
//! // Insert a subheading for each getter function signature
//!
//! \##### <code>\`example_getter_name()\`</code>
//!
//! // What it returns
//! // Why, when, and how often to call it
//! // When it could panic or error
//! // When safety issues to consider
//!
//! \#### Public Mutable functions (changing state)
//!
//! // Insert a subheading for each setter function signature
//!
//! \##### <code>\`example_setter_name(origin, parameter_name: T::ExampleType)\`</code>
//!
//! // What state it changes
//! // Why, when, and how often to call it
//! // When it could panic or error
//! // When safety issues to consider
//! // What parameter values are valid and why
//!
//! \### Storage Items
//!
//! // Explain any storage items included in this pallet
//!
//! \### Digest Items
//!
//! // Explain any digest items included in this pallet
//!
//! \### Inherent Data
//!
//! // Explain what inherent data (if any) is defined in the pallet and any other related types
//!
//! \### Events:
//!
//! // Insert events for this pallet if any
//!
//! \### Errors:
//!
//! // Explain what generates errors
//!
//! \## Usage
//!
//! // Insert 2-3 examples of usage and code snippets that show how to
//! // use <INSERT_CUSTOM_PALLET_NAME> Pallet in a custom pallet.
//!
//! \### Prerequisites
//!
//! // Show how to include necessary imports for <INSERT_CUSTOM_PALLET_NAME> and derive
//! // your pallet configuration trait with the `INSERT_CUSTOM_PALLET_NAME` trait.
//!
//! \```rust
//! use <INSERT_CUSTOM_PALLET_NAME>;
//!
//! pub trait Config: <INSERT_CUSTOM_PALLET_NAME>::Config { }
//! \```
//!
//! \### Simple Code Snippet
//!
//! // Show a simple example (e.g. how to query a public getter function of <INSERT_CUSTOM_PALLET_NAME>)
//!
//! \### Example from FRAME
//!
//! // Show a usage example in an actual runtime
//!
//! // See:
//! // - Substrate TCR <https://github.com/parity-samples/substrate-tcr>
//! // - Substrate Kitties <https://shawntabrizi.github.io/substrate-collectables-workshop/#/>
//!
//! \## Genesis Config
//!
//! <!-- Original author of paragraph: @joepetrowski -->
//!
//! \## Dependencies
//!
//! // Dependencies on other FRAME pallets and the genesis config should be mentioned,
//! // but not the Rust Standard Library.
//! // Genesis configuration modifications that may be made to incorporate this pallet
//! // Interaction with other pallets
//!
//! <!-- Original author of heading: @AmarRSingh -->
//!
//! \## Related Pallets
//!
//! // Interaction with other pallets in the form of a bullet point list
//!
//! \## References
//!
//! <!-- Original author of paragraph: @joepetrowski -->
//!
//! // Links to reference material, if applicable. For example, Phragmen, W3F research, etc.
//! // that the implementation is based on.
//! </pre></p></details>

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::marker::PhantomData;
use frame_support::{
	dispatch::DispatchResult, decl_module, decl_storage, decl_event, traits::IsSubType,
	weights::{DispatchClass, ClassifyDispatch, WeighData, Weight, PaysFee, Pays},
};
use sp_std::prelude::*;
use frame_system::{ensure_signed, ensure_root};
use codec::{Encode, Decode};
use sp_runtime::{
	traits::{
		SignedExtension, Bounded, SaturatedConversion, DispatchInfoOf,
	},
	transaction_validity::{
		ValidTransaction, TransactionValidityError, InvalidTransaction, TransactionValidity,
	},
};

/// Our pallet's configuration trait. All our types and constants go in here. If the
/// pallet is dependent on specific other pallets, then their configuration traits
/// should be added to our implied traits list.
///
/// `frame_system::Config` should always be included in our implied traits.
pub trait Config: frame_system::Config {
	/// The overarching event type.
	type Event: From<Event> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
	trait Store for Module<T: Config> as Example {

	}
}

decl_event!(
	/// Events are a simple means of reporting specific conditions and
	/// circumstances that have happened that users, Dapps and/or chain explorers would find
	/// interesting and otherwise difficult to detect.
	pub enum Event {
		// Just a normal `enum`, here's a dummy event to ensure it compiles.
		/// Dummy event, just here so there's a generic type that's used.
		Dummy,
	}
);

// The module declaration. This states the entry points that we handle. The
// macro takes care of the marshalling of arguments and dispatch.
//
// Anyone can have these functions execute by signing and submitting
// an extrinsic. Ensure that calls into each of these execute in a time, memory and
// using storage space proportional to any costs paid for by the caller or otherwise the
// difficulty of forcing the call to happen.
//
// Generally you'll want to split these into three groups:
// - Public calls that are signed by an external account.
// - Root calls that are allowed to be made only by the governance system.
// - Unsigned calls that can be of two kinds:
//   * "Inherent extrinsics" that are opinions generally held by the block
//     authors that build child blocks.
//   * Unsigned Transactions that are of intrinsic recognizable utility to the
//     network, and are validated by the runtime.
//
// Information about where this dispatch initiated from is provided as the first argument
// "origin". As such functions must always look like:
//
// `fn foo(origin, bar: Bar, baz: Baz) -> Result;`
//
// The `Result` is required as part of the syntax (and expands to the conventional dispatch
// result of `Result<(), &'static str>`).
//
// When you come to `impl` them later in the pallet, you must specify the full type for `origin`:
//
// `fn foo(origin: T::Origin, bar: Bar, baz: Baz) { ... }`
//
// There are three entries in the `frame_system::Origin` enum that correspond
// to the above bullets: `::Signed(AccountId)`, `::Root` and `::None`. You should always match
// against them as the first thing you do in your function. There are three convenience calls
// in system that do the matching for you and return a convenient result: `ensure_signed`,
// `ensure_root` and `ensure_none`.
decl_module! {
	// Simple declaration of the `Module` type. Lets the macro know what its working on.
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		/// Deposit one of this pallet's events by using the default implementation.
		/// It is also possible to provide a custom implementation.
		/// For non-generic events, the generic parameter just needs to be dropped, so that it
		/// looks like: `fn deposit_event() = default;`.
		fn deposit_event() = default;
	}
}

// The main implementation block for the pallet. Functions here fall into three broad
// categories:
// - Public interface. These are functions that are `pub` and generally fall into inspector
// functions that do not write to storage and operation functions that do.
// - Private functions. These are your usual private utilities unavailable to other pallets.
impl<T: Config> Module<T> {

}