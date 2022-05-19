//! Data for `load_metadata` and `add_specs` payloads and hot database updating
use constants::{COLOR, SECONDARY_COLOR};
use definitions::{
    crypto::Encryption,
    error_active::{
        Active, Changed, DatabaseActive, ErrorActive, Fetch, IncomingMetadataSourceActiveStr,
        NotHexActive,
    },
    helpers::unhex,
    metadata::MetaValues,
    network_specs::NetworkSpecsToSend,
};
use std::convert::TryInto;

use crate::fetch_metadata::{fetch_info, fetch_info_with_network_specs};
use crate::helpers::{filter_address_book_by_url, genesis_hash_in_hot_db, process_indices};
use crate::interpret_specs::interpret_properties;
use crate::parser::TokenOverride;

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

/// Data for `add_specs` payload
pub struct SpecsShortCut {
    /// network [`NetworkSpecsToSend`]
    pub specs: NetworkSpecsToSend,

    /// flag to indicate that exactly same entry is **not** yet in the database,
    /// and the database could be updated
    pub update: bool,
}

/// Get data needed for `add_specs` payload [`SpecsShortCut`] from given url
/// address and command line input
///
/// Note that this function uses the database, even if `-d` setting key is used.
pub fn specs_shortcut(
    address: &str,
    encryption: Encryption,
    optional_token_override: Option<TokenOverride>,
    optional_signer_title_override: Option<String>,
) -> Result<SpecsShortCut, ErrorActive> {
    // check entries in address book, to see if the url address is already known
    let entries = filter_address_book_by_url(address)?;

    // token override is allowed only if the network is entirely new and token
    // is not yet set
    if !entries.is_empty() && optional_token_override.is_some() {
        return Err(ErrorActive::NoTokenOverrideKnownNetwork {
            url: address.to_string(),
        });
    }

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

    // no entries in the address book for this url address
    if entries.is_empty() {
        // Nonetheless, there are some entries in `SPECSTREEPREP` with the same
        // genesis hash.
        //
        // Most likely, this has happened because network already is known to
        // the hot database, but with a different url address. At least for the
        // time being, having more than one url address for the same network is
        // not allowed.
        if genesis_hash_in_hot_db(genesis_hash)? {
            return Err(ErrorActive::Database(
                DatabaseActive::NewAddressKnownGenesisHash {
                    url: address.to_string(),
                    genesis_hash,
                },
            ));
        }

        let title = optional_signer_title_override.unwrap_or(format!(
            "{}-{}",
            meta_values.name,
            encryption.show()
        ));

        // Otherwise `NetworkSpecsToSend` entry is constructed with fetched and
        // user-entered values and with default colors.
        let specs = NetworkSpecsToSend {
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
        };
        Ok(SpecsShortCut {
            specs,
            update: true, // no entry in the database, can add it if requested
        })
    } else {
        // there are address book entries for the network;
        //
        // find a most suitable entry: same encryption as requested is the most
        // preferable, then the default entry, then any other entry
        //
        // tuple below is `specs` with appropriate title and encryption and
        // `update` flag to indicate if the entry is not yet in the database,
        // and could be added if requested
        let (specs, update) =
            process_indices(&entries, encryption, optional_signer_title_override)?;

        let url = address.to_string();

        // check that base58 prefix did not change
        if specs.base58prefix != new_properties.base58prefix {
            return Err(ErrorActive::Fetch(Fetch::ValuesChanged {
                url,
                what: Changed::Base58Prefix {
                    old: specs.base58prefix,
                    new: new_properties.base58prefix,
                },
            }));
        }

        // check that decimals did not change
        if specs.decimals != new_properties.decimals {
            return Err(ErrorActive::Fetch(Fetch::ValuesChanged {
                url,
                what: Changed::Decimals {
                    old: specs.decimals,
                    new: new_properties.decimals,
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
                    old: specs.name,
                    new: meta_values.name,
                },
            }));
        }

        // check that unit did not change
        if specs.unit != new_properties.unit {
            return Err(ErrorActive::Fetch(Fetch::ValuesChanged {
                url,
                what: Changed::Unit {
                    old: specs.unit,
                    new: new_properties.unit,
                },
            }));
        }

        // `NetworkSpecsToSend` are good, can use them
        Ok(SpecsShortCut { specs, update })
    }
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
