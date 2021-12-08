//
//  MKeys.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 8.12.2021.
//

import Foundation

struct MKeys: Decodable, Hashable {
    var keys: [MKeysKeyCard]
    var seed: String
}

struct MKeysKeyCard: Decodable, Hashable {
    var public_key: String
}
