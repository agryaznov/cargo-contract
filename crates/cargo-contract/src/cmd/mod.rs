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
        substrate::{
            BlakeTwo256,
            MultiAddress,
            MultiSignature,
            SubstrateExtrinsicParams,
            SubstrateHeader,
            H256,
        },
        Config,
    },
    OnlineClient,
};

pub enum PolkamaskConfig {}

impl Config for PolkamaskConfig {
    type Hash = H256;
    type AccountId = pmp_account::AccountId20;
    type Address = MultiAddress<Self::AccountId, u32>;
    type Signature = MultiSignature;
    type Hasher = BlakeTwo256;
    type Header = SubstrateHeader<u32, BlakeTwo256>;
    type ExtrinsicParams = SubstrateExtrinsicParams<Self>;
}

type Client = OnlineClient<PolkamaskConfig>;
type Balance = u128;
type CodeHash = <PolkamaskConfig as Config>::Hash;
