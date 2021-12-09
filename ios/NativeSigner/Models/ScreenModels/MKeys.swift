//
//  MKeys.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 8.12.2021.
//

import Foundation

struct MKeys: Decodable {
    var set: [MKeysKeyCard]
    var root: MSeedKeyCard
    var network: MNetworkCard
}

struct MKeysKeyCard: Decodable, Hashable {
    var address_key: String
    var base58: String
    var identicon: String
    var has_pwd: Bool
    var path: String
}

struct MSeedKeyCard: Decodable {
    var seed_name: String
    var identicon: String
    var address_key: String
    var base58: String
}

struct MNetworkCard: Decodable {
    var name: String
    var logo: String
}
