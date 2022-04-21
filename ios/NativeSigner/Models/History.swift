//
//  History.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 12.8.2021.
//

/**
 * This is hard-typed decoding of log passed from rust
 *
 * This should actually be reduced to very simple 4 or 5 filed object later,
 * as history screen has very simple cards
 *
 * Cards for history details are decoded in screen model but may pull some objects from here
 * Tread carefully until this mess is organized nicely
 */

import Foundation

struct CurrentVerifier: Decodable, Hashable {
    var type: String
    var details: Verifier
}

struct IdentityEvent: Decodable, Hashable {
    var seed_name: String
    var encryption: String
    var public_key: String
    var path: String
    var network_genesis_hash: String
}

struct NetworkDisplay: Decodable, Hashable {
    var base58prefix: String
    var color: String
    var decimals: String
    var encryption: String
    var genesis_hash: String
    var logo: String
    var name: String
    var order: String
    var path_id: String
    var secondary_color: String
    var title: String
    var unit: String
    var current_verifier: CurrentVerifier
}

struct NetworkSigned: Decodable, Hashable {
    var base58prefix: String
    var color: String
    var decimals: String
    var encryption: String
    var genesis_hash: String
    var logo: String
    var name: String
    var path_id: String
    var secondary_color: String
    var title: String
    var unit: String
    var signed_by: Verifier
}

struct SignMessage: Decodable, Hashable {
    var message: String
    var signed_by: Verifier
    var user_comment: String
}

struct SignMessageError: Decodable, Hashable {
    var message: String
    var signed_by: Verifier
    var user_comment: String
    var error: String
}

struct SignDisplayError: Decodable, Hashable {
    var transaction: String
    var signed_by: Verifier
    var user_comment: String
    var error: String
}

struct TypesSigned: Decodable, Hashable {
    var types_hash: String
    var signed_by: Verifier
}

struct MetadataSigned: Decodable, Hashable {
    var specname: String
    var spec_version: String
    var meta_hash: String
    var signed_by: Verifier
}
