//
//  MSeeds.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 8.12.2021.
//

import Foundation

struct MSeeds: Decodable, Hashable {
    var seedNameCards: [SeedNameCard]
}

struct SeedNameCard: Decodable, Hashable {
    var seed_name: String
    var identicon: String
}
