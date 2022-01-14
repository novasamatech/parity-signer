//
//  MNetworkDetails.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 20.12.2021.
//

import Foundation

struct MNetworkDetails: Decodable {
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
    var current_verifier: MVerifier
    var meta: [MMetadataRecord]
}

struct MVerifier: Decodable {
    var type: String
    var details: MVerifierDetails
}

struct MVerifierDetails: Decodable {
    var hex: String
    var identicon: String
    var encryption: String
}

struct MMetadataRecord: Decodable {
    var spec_version: String
    var meta_hash: String
    var meta_id_pic: String
}
