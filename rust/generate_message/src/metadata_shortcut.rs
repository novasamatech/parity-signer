//! Data for `load_metadata` and `add_specs` payloads and hot database updating
use constants::{COLOR, SECONDARY_COLOR};
use definitions::{
    crypto::Encryption,
    error_active::{
        Active, Changed, ErrorActive, Fetch, IncomingMetadataSourceActiveStr, NotHexActive,
        SpecsError,
    },
    helpers::unhex,
    metadata::MetaValues,
    network_specs::NetworkSpecsToSend,
};
use std::convert::TryInto;

use crate::fetch_metadata::{fetch_info, fetch_info_with_network_specs};
use crate::interpret_specs::{check_specs, interpret_properties, TokenFetch};
use crate::parser::Token;

/// Data for `load_metadata` payload
pub struct MetaShortCut {
    pub meta_values: MetaValues,
    pub genesis_hash: [u8; 32],
}

/// Get data needed for `load_metadata` payload [`MetaShortCut`] from given url
/// address
pub fn meta_shortcut(address: &str) -> Result<MetaShortCut, ErrorActive> {
    let new_info = match fetch_info(address) {
        Ok(a) => a,
        Err(e) => {
            return Err(ErrorActive::Fetch(Fetch::Failed {
                url: address.to_string(),
                error: e.to_string(),
            }))
        }
    };
    let genesis_hash = get_genesis_hash(address, &new_info.genesis_hash)?;
    let meta_values = MetaValues::from_str_metadata(
        &new_info.meta,
        IncomingMetadataSourceActiveStr::Fetch {
            url: address.to_string(),
        },
    )?;
    Ok(MetaShortCut {
        meta_values,
        genesis_hash,
    })
}

pub fn specs_agnostic(
    address: &str,
    encryption: Encryption,
    optional_token_override: Option<Token>,
    optional_signer_title_override: Option<String>,
) -> Result<NetworkSpecsToSend, ErrorActive> {
    // actual fetch
    let new_info = match fetch_info_with_network_specs(address) {
        Ok(a) => a,
        Err(e) => {
            return Err(ErrorActive::Fetch(Fetch::Failed {
                url: address.to_string(),
                error: e.to_string(),
            }))
        }
    };

    // genesis hash in proper format
    let genesis_hash = get_genesis_hash(address, &new_info.genesis_hash)?;

    // `MetaValues` are needed to get network name and (optionally) base58
    // prefix
    let meta_values = MetaValues::from_str_metadata(
        &new_info.meta,
        IncomingMetadataSourceActiveStr::Fetch {
            url: address.to_string(),
        },
    )?;

    // `NetworkProperties` checked and processed
    let new_properties = match interpret_properties(
        &new_info.properties,
        meta_values.optional_base58prefix,
        optional_token_override,
    ) {
        Ok(a) => a,
        Err(error) => {
            return Err(ErrorActive::Fetch(Fetch::FaultySpecs {
                url: address.to_string(),
                error,
            }))
        }
    };

    let title = optional_signer_title_override.unwrap_or(format!(
        "{}-{}",
        meta_values.name,
        encryption.show()
    ));

    // `NetworkSpecsToSend` is constructed with fetched and user-entered values
    // and with default colors.
    Ok(NetworkSpecsToSend {
        base58prefix: new_properties.base58prefix,
        color: COLOR.to_string(),
        decimals: new_properties.decimals,
        encryption,
        genesis_hash,
        logo: meta_values.name.to_string(),
        name: meta_values.name.to_string(),
        path_id: format!("//{}", meta_values.name),
        secondary_color: SECONDARY_COLOR.to_string(),
        title,
        unit: new_properties.unit,
    })
}

pub fn update_known_specs(
    address: &str,
    specs: &mut NetworkSpecsToSend,
    optional_signer_title_override: Option<String>,
    optional_token_override: Option<Token>,
) -> Result<bool, ErrorActive> {
    let mut update_done = false;
    let url = address.to_string();

    // actual fetch
    let new_info = match fetch_info_with_network_specs(address) {
        Ok(a) => a,
        Err(e) => {
            return Err(ErrorActive::Fetch(Fetch::Failed {
                url,
                error: e.to_string(),
            }))
        }
    };

    // genesis hash in proper format
    let genesis_hash = get_genesis_hash(address, &new_info.genesis_hash)?;

    // `MetaValues` are needed to get network name and (optionally) base58
    // prefix
    let meta_values = MetaValues::from_str_metadata(
        &new_info.meta,
        IncomingMetadataSourceActiveStr::Fetch {
            url: address.to_string(),
        },
    )?;

    let (base58prefix, token_fetch) =
        check_specs(&new_info.properties, meta_values.optional_base58prefix).map_err(|error| {
            ErrorActive::Fetch(Fetch::FaultySpecs {
                url: address.to_string(),
                error,
            })
        })?;

    // check that base58 prefix did not change
    if specs.base58prefix != base58prefix {
        return Err(ErrorActive::Fetch(Fetch::ValuesChanged {
            url,
            what: Changed::Base58Prefix {
                old: specs.base58prefix,
                new: base58prefix,
            },
        }));
    }

    // check that genesis hash did not change
    if specs.genesis_hash != genesis_hash {
        return Err(ErrorActive::Fetch(Fetch::ValuesChanged {
            url,
            what: Changed::GenesisHash {
                old: specs.genesis_hash,
                new: genesis_hash,
            },
        }));
    }

    // check that name did not change
    if specs.name != meta_values.name {
        return Err(ErrorActive::Fetch(Fetch::ValuesChanged {
            url,
            what: Changed::Name {
                old: specs.name.to_string(),
                new: meta_values.name,
            },
        }));
    }

    // check that token did not change or could be overridden
    match token_fetch {
        // single token, no override was or is possible, must match
        TokenFetch::Single(token) => {
            // check that decimals value did not change
            if specs.decimals != token.decimals {
                return Err(ErrorActive::Fetch(Fetch::ValuesChanged {
                    url,
                    what: Changed::Decimals {
                        old: specs.decimals,
                        new: token.decimals,
                    },
                }));
            }

            // check that unit did not change
            if specs.unit != token.unit {
                return Err(ErrorActive::Fetch(Fetch::ValuesChanged {
                    url,
                    what: Changed::Unit {
                        old: specs.unit.to_string(),
                        new: token.unit,
                    },
                }));
            }

            // override is not allowed
            if optional_token_override.is_some() {
                return Err(ErrorActive::Fetch(Fetch::FaultySpecs {
                    url,
                    error: SpecsError::OverrideIgnoredSingle,
                }));
            }
        }
        TokenFetch::Array { .. } => {
            // override is allowed
            if let Some(token) = optional_token_override {
                if specs.decimals != token.decimals {
                    specs.decimals = token.decimals;
                    update_done = true;
                }
                if specs.unit != token.unit {
                    specs.unit = token.unit;
                    update_done = true;
                }
            }
        }
        TokenFetch::None => {
            // only decimals `0` possible, check that decimals value did not
            // change
            if specs.decimals != 0 {
                return Err(ErrorActive::Fetch(Fetch::ValuesChanged {
                    url,
                    what: Changed::DecimalsBecameNone {
                        old: specs.decimals,
                    },
                }));
            }

            // only unit `UNIT` possible, check that unit did not change
            if specs.unit != "UNIT" {
                return Err(ErrorActive::Fetch(Fetch::ValuesChanged {
                    url,
                    what: Changed::UnitBecameNone {
                        old: specs.unit.to_string(),
                    },
                }));
            }

            // override is not allowed
            if optional_token_override.is_some() {
                return Err(ErrorActive::Fetch(Fetch::FaultySpecs {
                    url,
                    error: SpecsError::OverrideIgnoredNone,
                }));
            }
        }
    }

    if let Some(title) = optional_signer_title_override {
        if specs.title != title {
            specs.title = title;
            update_done = true;
        }
    }
    Ok(update_done)
}

pub fn update_modify_encryption_specs(
    address: &str,
    specs: &mut NetworkSpecsToSend,
    encryption: &Encryption,
    optional_signer_title_override: Option<String>,
    optional_token_override: Option<Token>,
) -> Result<(), ErrorActive> {
    let url = address.to_string();

    // actual fetch
    let new_info = match fetch_info_with_network_specs(address) {
        Ok(a) => a,
        Err(e) => {
            return Err(ErrorActive::Fetch(Fetch::Failed {
                url,
                error: e.to_string(),
            }))
        }
    };

    // genesis hash in proper format
    let genesis_hash = get_genesis_hash(address, &new_info.genesis_hash)?;

    // `MetaValues` are needed to get network name and (optionally) base58
    // prefix
    let meta_values = MetaValues::from_str_metadata(
        &new_info.meta,
        IncomingMetadataSourceActiveStr::Fetch {
            url: address.to_string(),
        },
    )?;

    let (base58prefix, token_fetch) =
        check_specs(&new_info.properties, meta_values.optional_base58prefix).map_err(|error| {
            ErrorActive::Fetch(Fetch::FaultySpecs {
                url: address.to_string(),
                error,
            })
        })?;

    // check that base58 prefix did not change
    if specs.base58prefix != base58prefix {
        return Err(ErrorActive::Fetch(Fetch::ValuesChanged {
            url,
            what: Changed::Base58Prefix {
                old: specs.base58prefix,
                new: base58prefix,
            },
        }));
    }

    // check that genesis hash did not change
    if specs.genesis_hash != genesis_hash {
        return Err(ErrorActive::Fetch(Fetch::ValuesChanged {
            url,
            what: Changed::GenesisHash {
                old: specs.genesis_hash,
                new: genesis_hash,
            },
        }));
    }

    // check that name did not change
    if specs.name != meta_values.name {
        return Err(ErrorActive::Fetch(Fetch::ValuesChanged {
            url,
            what: Changed::Name {
                old: specs.name.to_string(),
                new: meta_values.name,
            },
        }));
    }

    // check that token did not change or could be overridden
    match token_fetch {
        // single token, no override was or is possible, must match
        TokenFetch::Single(token) => {
            // check that unit did not change
            if specs.decimals != token.decimals {
                return Err(ErrorActive::Fetch(Fetch::ValuesChanged {
                    url,
                    what: Changed::Decimals {
                        old: specs.decimals,
                        new: token.decimals,
                    },
                }));
            }

            // check that unit did not change
            if specs.unit != token.unit {
                return Err(ErrorActive::Fetch(Fetch::ValuesChanged {
                    url,
                    what: Changed::Unit {
                        old: specs.unit.to_string(),
                        new: token.unit,
                    },
                }));
            }

            // override is not allowed
            if optional_token_override.is_some() {
                return Err(ErrorActive::Fetch(Fetch::FaultySpecs {
                    url,
                    error: SpecsError::OverrideIgnoredSingle,
                }));
            }
        }
        TokenFetch::Array { .. } => {
            // override is allowed
            if let Some(token) = optional_token_override {
                if specs.decimals != token.decimals {
                    specs.decimals = token.decimals;
                }
                if specs.unit != token.unit {
                    specs.unit = token.unit;
                }
            }
        }
        TokenFetch::None => {
            // override is not allowed
            if optional_token_override.is_some() {
                return Err(ErrorActive::Fetch(Fetch::FaultySpecs {
                    url,
                    error: SpecsError::OverrideIgnoredNone,
                }));
            }
        }
    }

    specs.title = optional_signer_title_override.unwrap_or(format!(
        "{}-{}",
        meta_values.name,
        encryption.show()
    ));

    specs.encryption = encryption.to_owned();

    Ok(())
}

/// Transform genesis hash from fetched hexadecimal string into proper format
///
/// Inputs url `address` from which the data was fetched and hex
/// `fetched_genesis_hash`.
// TODO fix genesis hash type if we fix genesis hash type after all
fn get_genesis_hash(address: &str, fetched_genesis_hash: &str) -> Result<[u8; 32], ErrorActive> {
    let genesis_hash_vec = unhex::<Active>(
        fetched_genesis_hash,
        NotHexActive::FetchedGenesisHash {
            url: address.to_string(),
        },
    )?;
    let out: [u8; 32] = match genesis_hash_vec.try_into() {
        Ok(a) => a,
        Err(_) => {
            return Err(ErrorActive::Fetch(
                Fetch::UnexpectedFetchedGenesisHashFormat {
                    value: fetched_genesis_hash.to_string(),
                },
            ))
        }
    };
    Ok(out)
}
