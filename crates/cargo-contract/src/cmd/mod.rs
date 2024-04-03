// Copyright 2018-2022 Parity Technologies (UK) Ltd.
// This file is part of cargo-contract.
//
// cargo-contract is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// cargo-contract is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with cargo-contract.  If not, see <http://www.gnu.org/licenses/>.

pub mod build;
pub mod decode;
pub mod encode;
pub mod info;
pub mod runtime_api;

pub(crate) use self::{
    build::{
        BuildCommand,
        CheckCommand,
    },
    decode::DecodeCommand,
    info::InfoCommand,
};
mod extrinsics;

pub(crate) use self::extrinsics::{
    CallCommand,
    ErrorVariant,
    InstantiateCommand,
    RemoveCommand,
    UploadCommand,
};

use subxt::{
    config::{
        polkadot::PolkadotExtrinsicParams,
        substrate::{
            BlakeTwo256,
            MultiAddress,
            SubstrateHeader,
            H256,
        },
        Config,
    },
    OnlineClient,
};

use ep_crypto::EthereumSignature;
pub use subxt::utils::AccountId20;

#[derive(Debug)]
pub enum PolkamaskConfig {}

impl Config for PolkamaskConfig {
    type Hash = H256;
    type AccountId = AccountId20;
    type Address = MultiAddress<Self::AccountId, u32>;
    type Signature = EthereumSignature;
    type Hasher = BlakeTwo256;
    type Header = SubstrateHeader<u32, BlakeTwo256>;
    type ExtrinsicParams = PolkadotExtrinsicParams<Self>; // PolkamaskExtrinsicParams<Self>;
}

type Client = OnlineClient<PolkamaskConfig>;
type Balance = u128;
type CodeHash = <PolkamaskConfig as Config>::Hash;

// TODO put into separate module\crate or remove
// use std::marker::PhantomData;
// use subxt::config::extrinsic_params::ExtrinsicParams;
// use scale::Encode;
// We add this dummy type in order to make the cargo-contract tool compatible with our
// simplified development node which has no added signed extentions.
// This is temporary means for the ease of development.
// #[derive(Debug)]
// pub struct PolkamaskExtrinsicParams<T> {
//     spec_version: u32,
//     transaction_version: u32,
//     phantom: PhantomData<T>,
// }

// impl<T: Config + std::fmt::Debug> ExtrinsicParams<T::Hash>
//     for PolkamaskExtrinsicParams<T>
// {
//     type OtherParams = ();

//     fn new(
//         spec_version: u32,
//         transaction_version: u32,
//         _nonce: u64,
//         _genesis_hash: T::Hash,
//         _other_params: Self::OtherParams,
//     ) -> Self {
//         PolkamaskExtrinsicParams {
//             spec_version,
//             transaction_version,
//             phantom: PhantomData::<T>,
//         }
//     }

//     fn encode_extra_to(&self, _v: &mut Vec<u8>) {}

//     fn encode_additional_to(&self, _v: &mut Vec<u8>) {}
// }
