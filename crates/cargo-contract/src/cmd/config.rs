// Copyright (C) Use Ink (UK) Ltd.
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

use ink_env::{
    DefaultEnvironment,
    Environment,
    NoChainExtension,
};
use std::{
    fmt::Debug,
    str::FromStr,
};
use subxt::{
    config::{
        substrate::{
            BlakeTwo256,
            MultiAddress,
            SubstrateHeader,
            H256,
        },
        PolkadotExtrinsicParams,
        SubstrateExtrinsicParams,
    },
    ext::{
        sp_core,
        sp_core::Pair,
    },
    tx::{
        PairSigner,
        Signer as SignerT,
    },
    utils::AccountId20,
    Config,
    PolkadotConfig,
    SubstrateConfig,
};

use subxt_signer::{
    ecdsa,
    SecretUri,
};

use ep_eth::EthereumSignature;

/// Configuration for signer
pub trait SignerConfig<C: Config + Environment> {
    type Signer: SignerT<C> + FromStr + Clone;
}

/// A runtime configuration for a Ethink chain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ecdsachain {}

impl Config for Ecdsachain {
    type Hash = <SubstrateConfig as Config>::Hash;
    type AccountId = <SubstrateConfig as Config>::AccountId;
    type Address = <SubstrateConfig as Config>::Address;
    type Signature = EthereumSignature;
    type Hasher = <SubstrateConfig as Config>::Hasher;
    type Header = <SubstrateConfig as Config>::Header;
    type ExtrinsicParams = SubstrateExtrinsicParams<Self>;
    type AssetId = <SubstrateConfig as Config>::AssetId;
}

impl Environment for Ecdsachain {
    const MAX_EVENT_TOPICS: usize = <DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;
    type AccountId = <DefaultEnvironment as Environment>::AccountId;
    type Balance = <DefaultEnvironment as Environment>::Balance;
    type Hash = <DefaultEnvironment as Environment>::Hash;
    type Timestamp = <DefaultEnvironment as Environment>::Timestamp;
    type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;
    type ChainExtension = <DefaultEnvironment as Environment>::ChainExtension;
}

impl SignerConfig<Self> for Ecdsachain {
    type Signer = SignerEcdsa;
}

/// A runtime configuration for the Substrate based chain.
/// This thing is not meant to be instantiated; it is just a collection of types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Substrate {}

impl Config for Substrate {
    type Hash = <SubstrateConfig as Config>::Hash;
    type AccountId = <SubstrateConfig as Config>::AccountId;
    type Address = <SubstrateConfig as Config>::Address;
    type Signature = <SubstrateConfig as Config>::Signature;
    type Hasher = <SubstrateConfig as Config>::Hasher;
    type Header = <SubstrateConfig as Config>::Header;
    type ExtrinsicParams = SubstrateExtrinsicParams<Self>;
    type AssetId = <SubstrateConfig as Config>::AssetId;
}

impl Environment for Substrate {
    const MAX_EVENT_TOPICS: usize = <DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;
    type AccountId = <DefaultEnvironment as Environment>::AccountId;
    type Balance = <DefaultEnvironment as Environment>::Balance;
    type Hash = <DefaultEnvironment as Environment>::Hash;
    type Timestamp = <DefaultEnvironment as Environment>::Timestamp;
    type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;
    type ChainExtension = <DefaultEnvironment as Environment>::ChainExtension;
}

impl SignerConfig<Self> for Substrate {
    type Signer = SignerEcdsa;
}

/// A runtime configuration for the Polkadot based chain.
/// This thing is not meant to be instantiated; it is just a collection of types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Polkadot {}

impl Config for Polkadot {
    type Hash = <PolkadotConfig as Config>::Hash;
    type AccountId = <PolkadotConfig as Config>::AccountId;
    type Address = <PolkadotConfig as Config>::Address;
    type Signature = <PolkadotConfig as Config>::Signature;
    type Hasher = <PolkadotConfig as Config>::Hasher;
    type Header = <PolkadotConfig as Config>::Header;
    type ExtrinsicParams = PolkadotExtrinsicParams<Self>;
    type AssetId = <PolkadotConfig as Config>::AssetId;
}

impl Environment for Polkadot {
    const MAX_EVENT_TOPICS: usize = <DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;
    type AccountId = <DefaultEnvironment as Environment>::AccountId;
    type Balance = <DefaultEnvironment as Environment>::Balance;
    type Hash = <DefaultEnvironment as Environment>::Hash;
    type Timestamp = <DefaultEnvironment as Environment>::Timestamp;
    type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;
    type ChainExtension = <DefaultEnvironment as Environment>::ChainExtension;
}

impl SignerConfig<Self> for Polkadot {
    type Signer = SignerEcdsa;
}

/// Struct representing the implementation of the sr25519 signer
#[derive(Clone)]
pub struct SignerSR25519<C: Config>(pub PairSigner<C, sp_core::sr25519::Pair>);

impl<C: Config> FromStr for SignerSR25519<C>
where
    <C as Config>::AccountId: From<sp_core::crypto::AccountId32>,
{
    type Err = anyhow::Error;

    /// Attempts to parse the Signer suri string
    fn from_str(input: &str) -> Result<SignerSR25519<C>, Self::Err> {
        let keypair = sp_core::sr25519::Pair::from_string(input, None)?;
        let signer = PairSigner::<C, _>::new(keypair);
        Ok(Self(signer))
    }
}

impl<C: Config> SignerT<C> for SignerSR25519<C>
where
    <C as Config>::Signature: From<sp_core::sr25519::Signature>,
{
    fn account_id(&self) -> <C as Config>::AccountId {
        self.0.account_id().clone()
    }

    fn address(&self) -> C::Address {
        self.0.address()
    }

    fn sign(&self, signer_payload: &[u8]) -> C::Signature {
        self.0.sign(signer_payload)
    }
}

/// Struct representing the implementation of the ecdsa signer
#[derive(Clone)]
pub struct SignerEcdsa(pub ecdsa::Keypair);

impl FromStr for SignerEcdsa {
    type Err = anyhow::Error;

    /// Attempts to parse the Signer suri string
    fn from_str(input: &str) -> Result<SignerEcdsa, Self::Err> {
        let suri = SecretUri::from_str(input)?;
        let signer = ecdsa::Keypair::from_uri(&suri)?;
        println!("READING SIGNER FROM SURI");
        Ok(Self(signer))
    }
}

impl<C: Config> SignerT<C> for SignerEcdsa
where
    <C as Config>::AccountId: From<ecdsa::PublicKey>,
    <C as Config>::Address: From<ecdsa::PublicKey>,
    <C as Config>::Signature: From<ecdsa::Signature>,
{
    fn account_id(&self) -> <C as Config>::AccountId {
        println!("Getting AccountId from signer");
        <ecdsa::Keypair as SignerT<C>>::account_id(&self.0).clone()
    }

    fn address(&self) -> C::Address {
        <ecdsa::Keypair as SignerT<C>>::address(&self.0)
    }

    fn sign(&self, signer_payload: &[u8]) -> C::Signature {
        <ecdsa::Keypair as SignerT<C>>::sign(&self.0, signer_payload)
    }
}

#[macro_export]
macro_rules! call_with_config_internal {
    ($obj:tt ,$function:tt, $config_name:expr, $($config:ty),*) => {
        match $config_name {
            $(
                stringify!($config) => $obj.$function::<$config>().await,
            )*
            _ => {

                let configs = vec![$(stringify!($config)),*].iter()
                .map(|s| s.trim_start_matches("crate::cmd::config::"))
                .collect::<Vec<_>>()
                .join(", ");
                Err(ErrorVariant::Generic(
                    contract_extrinsics::GenericError::from_message(
                        format!("Chain configuration not found, Allowed configurations: {configs}")
                )))
            },
        }
    };
}

/// Macro that allows calling the command member function with chain configuration
#[macro_export]
macro_rules! call_with_config {
    ($obj:tt, $function:ident, $config_name:expr) => {{
        let config_name = format!("crate::cmd::config::{}", $config_name);
        $crate::call_with_config_internal!(
            $obj,
            $function,
            config_name.as_str(),
            // All available chain configs need to be specified here
            $crate::cmd::config::Polkadot,
            $crate::cmd::config::Substrate,
            $crate::cmd::config::Ecdsachain
        )
    }};
}
