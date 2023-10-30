// Copyright (C) 2021 Subspace Labs, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Subspace chain configurations.

use crate::chain_spec_utils::{
    chain_spec_properties, get_account_id_from_seed, get_public_key_from_seed,
};
use crate::domain::evm_chain_spec::{self, SpecId};
use hex_literal::hex;
use parity_scale_codec::Encode;
use sc_service::{ChainType, NoExtension};
use sc_subspace_chain_specs::ConsensusChainSpec;
use sc_telemetry::TelemetryEndpoints;
use sp_consensus_subspace::FarmerPublicKey;
use sp_core::crypto::{Ss58Codec, UncheckedFrom};
use sp_domains::storage::RawGenesis;
use sp_domains::{OperatorAllowList, OperatorPublicKey, RuntimeType};
use sp_runtime::{BuildStorage, Percent};
use std::collections::btree_set::BTreeSet;
use std::marker::PhantomData;
use std::num::NonZeroU32;
use subspace_core_primitives::PotKey;
use subspace_runtime::{
    AllowAuthoringBy, BalancesConfig, DomainsConfig, MaxDomainBlockSize, MaxDomainBlockWeight,
    RuntimeConfigsConfig, RuntimeGenesisConfig, SubspaceConfig, SudoConfig, SystemConfig,
    VestingConfig, MILLISECS_PER_BLOCK, WASM_BINARY,
};
use subspace_runtime_primitives::{AccountId, Balance, BlockNumber, SSC};

const SUBSPACE_TELEMETRY_URL: &str = "wss://telemetry.subspace.network/submit/";
const DEVNET_CHAIN_SPEC: &[u8] = include_bytes!("../res/chain-spec-raw-devnet.json");

/// List of accounts which should receive token grants, amounts are specified in SSC.
const TOKEN_GRANTS: &[(&str, u128)] = &[
    (
        "5Dns1SVEeDqnbSm2fVUqHJPCvQFXHVsgiw28uMBwmuaoKFYi",
        3_000_000,
    ),
    (
        "5DxtHHQL9JGapWCQARYUAWj4yDcwuhg9Hsk5AjhEzuzonVyE",
        1_500_000,
    ),
    ("5EHhw9xuQNdwieUkNoucq2YcateoMVJQdN8EZtmRy3roQkVK", 133_333),
    ("5GBWVfJ253YWVPHzWDTos1nzYZpa9TemP7FpQT9RnxaFN6Sz", 350_000),
    ("5F9tEPid88uAuGbjpyegwkrGdkXXtaQ9sGSWEnYrfVCUCsen", 111_111),
    ("5DkJFCv3cTBsH5y1eFT94DXMxQ3EmVzYojEA88o56mmTKnMp", 244_444),
    ("5G23o1yxWgVNQJuL4Y9UaCftAFvLuMPCRe7BCARxCohjoHc9", 311_111),
    ("5GhHwuJoK1b7uUg5oi8qUXxWHdfgzv6P5CQSdJ3ffrnPRgKM", 317_378),
    ("5D9pNnGCiZ9UqhBQn5n71WFVaRLvZ7znsMvcZ7PHno4zsiYa", 600_000),
    ("5H2Kq1qWrisf7aXUvdGrQB9j9zhiGt6MdaGSSBpFCwynBT9p", 34_950),
    ("5Ci12WM1YqPjSAMNubucNejuSqwChfRSKDpFfFhtshomNSG1", 250_000),
    ("5DydwBX2uLjnVKjg1zAWS3z27ukbr99PiXteQSg96bb1k6p7", 40_000),
    ("5FAS1mdyp1yomAzJaJ74ZgJbzicQmZ8ajRyxPZ2x4wseGkY2", 104_175),
    ("5E4vk2Ant4y6KiKoGMezrhhFwSanspjh8Fxa9HmWmjWrFyry", 66_700),
    ("5GsCx12U1zMu7bMZHXjb1rhMFR8YK9VUj6hQHWyaw1ReYt8D", 33_333),
    ("5F72mz79TjkWQEjuefPCMabFarGVLvW4haPTYsrzewxrbuD7", 12_222),
    ("5Fn9BF7pyiefhAwanXFyW4T5sXNQGJ9kzLAR1DpF8iYmc7aw", 6_667),
    ("5CdMyLvrxdTNTVZYAgN9NCQbNmwYW32vojsBZZfkEcxYbjUR", 33_333),
    ("5G4BCrTj6xZHkTwFtPmK4sjNEXc8w12ZjLLU8awsb5CDBz4d", 10_000),
    ("5FND87MkPEVvwMP3sq88n1MFxHuLDrHkBdCNeuc23ibjHME4", 38_889),
    ("5Fgjk1nMYCEcoP9QvjMDVzDjBzJfo5X2ZssSbWn5PesfyPJh", 100_000),
    ("5CUutLkRAMr14dsqFzRFgByD6gv9U8iqL67CZ7huxXtoXKdB", 22_222),
    ("5EqPLjjBob7Y6FCUMKMPfgQyb2BZ8y2CcNVQrZ5wSF3aDpTX", 3_333),
    ("5DXfPcXUcP4BG8LBSkJDrfFNApxjWySR6ARfgh3v27hdYr5S", 440_000),
    ("5CXSdDJgzRTj54f9raHN2Z5BNPSMa2ETjqCTUmpaw3ECmwm4", 330_000),
    ("5DqKxL7bQregQmUfFgzTMfRKY4DSvA1KgHuurZWYmxYSCmjY", 200_000),
    ("5CfixiS93yTwHQbzzfn8P2tMxhKXdTx7Jam9htsD7XtiMFtn", 27_800),
    ("5FZe9YzXeEXe7sK5xLR8yCmbU8bPJDTZpNpNbToKvSJBUiEo", 18_067),
    ("5FZwEgsvZz1vpeH7UsskmNmTpbfXvAcojjgVfShgbRqgC1nx", 27_800),
    ("5EqBwtqrCV427xCtTsxnb9X2Qay39pYmKNk9wD9Kd62jLS97", 75_000),
];

/// Additional subspace specific genesis parameters.
struct GenesisParams {
    enable_rewards: bool,
    enable_storage_access: bool,
    allow_authoring_by: AllowAuthoringBy,
    pot_slot_iterations: NonZeroU32,
    enable_domains: bool,
    enable_balance_transfers: bool,
    confirmation_depth_k: u32,
}

struct GenesisDomainParams {
    domain_name: String,
    operator_allow_list: OperatorAllowList<AccountId>,
    operator_signing_key: OperatorPublicKey,
}

pub fn gemini_3g_compiled() -> Result<ConsensusChainSpec<RuntimeGenesisConfig>, String> {
    Ok(ConsensusChainSpec::from_genesis(
        // Name
        "Subspace Gemini 3g",
        // ID
        "subspace_gemini_3g",
        ChainType::Custom("Subspace Gemini 3g".to_string()),
        || {
            let sudo_account =
                AccountId::from_ss58check("5DNwQTHfARgKoa2NdiUM51ZUow7ve5xG9S2yYdSbVQcnYxBA")
                    .expect("Wrong root account address");

            let mut balances = vec![(sudo_account.clone(), 1_000 * SSC)];
            let vesting_schedules = TOKEN_GRANTS
                .iter()
                .flat_map(|&(account_address, amount)| {
                    let account_id = AccountId::from_ss58check(account_address)
                        .expect("Wrong vesting account address");
                    let amount: Balance = amount * SSC;

                    // TODO: Adjust start block to real value before mainnet launch
                    let start_block = 100_000_000;
                    let one_month_in_blocks =
                        u32::try_from(3600 * 24 * 30 * MILLISECS_PER_BLOCK / 1000)
                            .expect("One month of blocks always fits in u32; qed");

                    // Add balance so it can be locked
                    balances.push((account_id.clone(), amount));

                    [
                        // 1/4 of tokens are released after 1 year.
                        (
                            account_id.clone(),
                            start_block,
                            one_month_in_blocks * 12,
                            1,
                            amount / 4,
                        ),
                        // 1/48 of tokens are released every month after that for 3 more years.
                        (
                            account_id,
                            start_block + one_month_in_blocks * 12,
                            one_month_in_blocks,
                            36,
                            amount / 48,
                        ),
                    ]
                })
                .collect::<Vec<_>>();
            subspace_genesis_config(
                SpecId::Gemini,
                WASM_BINARY.expect("Wasm binary must be built for Gemini"),
                sudo_account.clone(),
                balances,
                vesting_schedules,
                GenesisParams {
                    enable_rewards: false,
                    enable_storage_access: false,
                    allow_authoring_by: AllowAuthoringBy::RootFarmer(
                        FarmerPublicKey::unchecked_from(hex_literal::hex!(
                            "8aecbcf0b404590ddddc01ebacb205a562d12fdb5c2aa6a4035c1a20f23c9515"
                        )),
                    ),
                    // TODO: Adjust once we bench PoT on faster hardware
                    // About 1s on 6.0 GHz Raptor Lake CPU (14900K)
                    pot_slot_iterations: NonZeroU32::new(200_032_000).expect("Not zero; qed"),
                    enable_domains: true,
                    enable_balance_transfers: true,
                    confirmation_depth_k: 100, // TODO: Proper value here
                },
                GenesisDomainParams {
                    domain_name: "nova".to_owned(),
                    operator_allow_list: OperatorAllowList::Operators(BTreeSet::from_iter(vec![
                        sudo_account,
                    ])),
                    operator_signing_key: OperatorPublicKey::unchecked_from(hex!(
                        "aa3b05b4d649666723e099cf3bafc2f2c04160ebe0e16ddc82f72d6ed97c4b6b"
                    )),
                },
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        Some(
            TelemetryEndpoints::new(vec![(SUBSPACE_TELEMETRY_URL.into(), 1)])
                .map_err(|error| error.to_string())?,
        ),
        // Protocol ID
        Some("subspace-gemini-3g"),
        None,
        // Properties
        Some({
            let mut properties = chain_spec_properties();
            properties.insert(
                "potExternalEntropy".to_string(),
                serde_json::to_value(None::<PotKey>).expect("Serialization is not infallible; qed"),
            );
            properties
        }),
        // Extensions
        NoExtension::None,
    ))
}

pub fn gemini_3g_config() -> Result<ConsensusChainSpec<RuntimeGenesisConfig>, String> {
    unimplemented!("Please use release prefixed with Gemini-3g.")
}

pub fn devnet_config() -> Result<ConsensusChainSpec<RuntimeGenesisConfig>, String> {
    ConsensusChainSpec::from_json_bytes(DEVNET_CHAIN_SPEC)
}

pub fn devnet_config_compiled() -> Result<ConsensusChainSpec<RuntimeGenesisConfig>, String> {
    Ok(ConsensusChainSpec::from_genesis(
        // Name
        "Subspace Dev network",
        // ID
        "subspace_devnet",
        ChainType::Custom("Testnet".to_string()),
        || {
            let sudo_account =
                AccountId::from_ss58check("5CXTmJEusve5ixyJufqHThmy4qUrrm6FyLCR7QfE4bbyMTNC")
                    .expect("Wrong root account address");

            let mut balances = vec![(sudo_account.clone(), 1_000 * SSC)];
            let vesting_schedules = TOKEN_GRANTS
                .iter()
                .flat_map(|&(account_address, amount)| {
                    let account_id = AccountId::from_ss58check(account_address)
                        .expect("Wrong vesting account address");
                    let amount: Balance = amount * SSC;

                    // TODO: Adjust start block to real value before mainnet launch
                    let start_block = 100_000_000;
                    let one_month_in_blocks =
                        u32::try_from(3600 * 24 * 30 * MILLISECS_PER_BLOCK / 1000)
                            .expect("One month of blocks always fits in u32; qed");

                    // Add balance so it can be locked
                    balances.push((account_id.clone(), amount));

                    [
                        // 1/4 of tokens are released after 1 year.
                        (
                            account_id.clone(),
                            start_block,
                            one_month_in_blocks * 12,
                            1,
                            amount / 4,
                        ),
                        // 1/48 of tokens are released every month after that for 3 more years.
                        (
                            account_id,
                            start_block + one_month_in_blocks * 12,
                            one_month_in_blocks,
                            36,
                            amount / 48,
                        ),
                    ]
                })
                .collect::<Vec<_>>();
            subspace_genesis_config(
                SpecId::DevNet,
                WASM_BINARY.expect("Wasm binary must be built for Gemini"),
                sudo_account,
                balances,
                vesting_schedules,
                GenesisParams {
                    enable_rewards: false,
                    enable_storage_access: false,
                    allow_authoring_by: AllowAuthoringBy::FirstFarmer,
                    pot_slot_iterations: NonZeroU32::new(150_000_000).expect("Not zero; qed"),
                    enable_domains: true,
                    enable_balance_transfers: true,
                    confirmation_depth_k: 100, // TODO: Proper value here
                },
                GenesisDomainParams {
                    domain_name: "evm-domain".to_owned(),
                    operator_allow_list: OperatorAllowList::Anyone,
                    operator_signing_key: OperatorPublicKey::unchecked_from(hex!(
                        "aa3b05b4d649666723e099cf3bafc2f2c04160ebe0e16ddc82f72d6ed97c4b6b"
                    )),
                },
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        Some(
            TelemetryEndpoints::new(vec![(SUBSPACE_TELEMETRY_URL.into(), 1)])
                .map_err(|error| error.to_string())?,
        ),
        // Protocol ID
        Some("subspace-devnet"),
        None,
        // Properties
        Some({
            let mut properties = chain_spec_properties();
            properties.insert(
                "potExternalEntropy".to_string(),
                serde_json::to_value(None::<PotKey>).expect("Serialization is not infallible; qed"),
            );
            properties
        }),
        // Extensions
        NoExtension::None,
    ))
}

pub fn dev_config() -> Result<ConsensusChainSpec<RuntimeGenesisConfig>, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ConsensusChainSpec::from_genesis(
        // Name
        "Subspace development",
        // ID
        "subspace_dev",
        ChainType::Development,
        || {
            subspace_genesis_config(
                SpecId::Dev,
                wasm_binary,
                // Sudo account
                get_account_id_from_seed("Alice"),
                // Pre-funded accounts
                vec![
                    (get_account_id_from_seed("Alice"), 1_000 * SSC),
                    (get_account_id_from_seed("Bob"), 1_000 * SSC),
                    (get_account_id_from_seed("Alice//stash"), 1_000 * SSC),
                    (get_account_id_from_seed("Bob//stash"), 1_000 * SSC),
                ],
                vec![],
                GenesisParams {
                    enable_rewards: false,
                    enable_storage_access: false,
                    allow_authoring_by: AllowAuthoringBy::Anyone,
                    pot_slot_iterations: NonZeroU32::new(100_000_000).expect("Not zero; qed"),
                    enable_domains: true,
                    enable_balance_transfers: true,
                    confirmation_depth_k: 5,
                },
                GenesisDomainParams {
                    domain_name: "evm-domain".to_owned(),
                    operator_allow_list: OperatorAllowList::Anyone,
                    operator_signing_key: get_public_key_from_seed::<OperatorPublicKey>("Alice"),
                },
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        None,
        // Properties
        Some({
            let mut properties = chain_spec_properties();
            properties.insert(
                "potExternalEntropy".to_string(),
                serde_json::to_value(None::<PotKey>).expect("Serialization is not infallible; qed"),
            );
            properties
        }),
        // Extensions
        NoExtension::None,
    ))
}

pub fn local_config() -> Result<ConsensusChainSpec<RuntimeGenesisConfig>, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ConsensusChainSpec::from_genesis(
        // Name
        "Subspace local",
        // ID
        "subspace_local",
        ChainType::Local,
        || {
            subspace_genesis_config(
                SpecId::Local,
                wasm_binary,
                // Sudo account
                get_account_id_from_seed("Alice"),
                // Pre-funded accounts
                vec![
                    (get_account_id_from_seed("Alice"), 1_000 * SSC),
                    (get_account_id_from_seed("Bob"), 1_000 * SSC),
                    (get_account_id_from_seed("Charlie"), 1_000 * SSC),
                    (get_account_id_from_seed("Dave"), 1_000 * SSC),
                    (get_account_id_from_seed("Eve"), 1_000 * SSC),
                    (get_account_id_from_seed("Ferdie"), 1_000 * SSC),
                    (get_account_id_from_seed("Alice//stash"), 1_000 * SSC),
                    (get_account_id_from_seed("Bob//stash"), 1_000 * SSC),
                    (get_account_id_from_seed("Charlie//stash"), 1_000 * SSC),
                    (get_account_id_from_seed("Dave//stash"), 1_000 * SSC),
                    (get_account_id_from_seed("Eve//stash"), 1_000 * SSC),
                    (get_account_id_from_seed("Ferdie//stash"), 1_000 * SSC),
                ],
                vec![],
                GenesisParams {
                    enable_rewards: false,
                    enable_storage_access: false,
                    allow_authoring_by: AllowAuthoringBy::Anyone,
                    pot_slot_iterations: NonZeroU32::new(100_000_000).expect("Not zero; qed"),
                    enable_domains: true,
                    enable_balance_transfers: true,
                    confirmation_depth_k: 1,
                },
                GenesisDomainParams {
                    domain_name: "evm-domain".to_owned(),
                    operator_allow_list: OperatorAllowList::Anyone,
                    operator_signing_key: get_public_key_from_seed::<OperatorPublicKey>("Alice"),
                },
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        None,
        // Properties
        Some({
            let mut properties = chain_spec_properties();
            properties.insert(
                "potExternalEntropy".to_string(),
                serde_json::to_value(None::<PotKey>).expect("Serialization is not infallible; qed"),
            );
            properties
        }),
        // Extensions
        NoExtension::None,
    ))
}

/// Configure initial storage state for FRAME modules.
fn subspace_genesis_config(
    spec_id: SpecId,
    wasm_binary: &[u8],
    sudo_account: AccountId,
    balances: Vec<(AccountId, Balance)>,
    // who, start, period, period_count, per_period
    vesting: Vec<(AccountId, BlockNumber, BlockNumber, u32, Balance)>,
    genesis_params: GenesisParams,
    genesis_domain_params: GenesisDomainParams,
) -> RuntimeGenesisConfig {
    let GenesisParams {
        enable_rewards,
        enable_storage_access,
        allow_authoring_by,
        pot_slot_iterations,
        enable_domains,
        enable_balance_transfers,
        confirmation_depth_k,
    } = genesis_params;

    let raw_genesis_storage = {
        let domain_genesis_config = evm_chain_spec::get_testnet_genesis_by_spec_id(spec_id);
        let storage = domain_genesis_config
            .build_storage()
            .expect("Failed to build genesis storage from genesis runtime config");
        let raw_genesis = RawGenesis::from_storage(storage);
        raw_genesis.encode()
    };

    RuntimeGenesisConfig {
        system: SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
            ..Default::default()
        },
        balances: BalancesConfig { balances },
        transaction_payment: Default::default(),
        sudo: SudoConfig {
            // Assign network admin rights.
            key: Some(sudo_account.clone()),
        },
        subspace: SubspaceConfig {
            enable_rewards,
            enable_storage_access,
            allow_authoring_by,
            pot_slot_iterations,
            phantom: PhantomData,
        },
        vesting: VestingConfig { vesting },
        runtime_configs: RuntimeConfigsConfig {
            enable_domains,
            enable_balance_transfers,
            confirmation_depth_k,
        },
        domains: DomainsConfig {
            genesis_domain: Some(sp_domains::GenesisDomain {
                runtime_name: "evm".to_owned(),
                runtime_type: RuntimeType::Evm,
                runtime_version: evm_domain_runtime::VERSION,
                raw_genesis_storage,

                // Domain config, mainly for placeholder the concrete value TBD
                owner_account_id: sudo_account,
                domain_name: genesis_domain_params.domain_name,
                max_block_size: MaxDomainBlockSize::get(),
                max_block_weight: MaxDomainBlockWeight::get(),
                bundle_slot_probability: (1, 1),
                target_bundles_per_block: 10,
                operator_allow_list: genesis_domain_params.operator_allow_list,
                signing_key: genesis_domain_params.operator_signing_key,
                nomination_tax: Percent::from_percent(5),
                minimum_nominator_stake: 100 * SSC,
            }),
        },
    }
}
