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
    var multiselect_mode: Bool
    var multiselect_count: String
}

struct MKeysKeyCard: Decodable, Hashable {
    var address_key: String
    var base58: String
    var identicon: String
    var has_pwd: Bool
    var path: String
    var swiped: Bool
    var multiselect: Bool
    
    func intoAddress() -> Address {
        return Address(
            base58: self.base58,
            path: self.path,
            has_pwd: self.has_pwd,
            identicon: self.identicon,
            seed_name: "",
            multiselect: self.multiselect
        )
    }
}

struct MSeedKeyCard: Decodable {
    var seed_name: String
    var identicon: String
    var address_key: String
    var base58: String
    var swiped: Bool
    var multiselect: Bool
}

struct MNetworkCard: Decodable {
    var title: String
    var logo: String
}
