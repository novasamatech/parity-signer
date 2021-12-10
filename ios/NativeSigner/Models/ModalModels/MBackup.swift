//
//  MBackup.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 10.12.2021.
//

import Foundation

struct MBackup: Decodable, Equatable {
    var seed_name: String
    var derivations: [DerivationsPack]
}

struct DerivationsPack: Decodable, Hashable {
    var network_title: String
    var network_logo: String
    var network_order: Int
    var id_set: [DerivationEntry]
}

struct DerivationEntry: Decodable, Hashable {
    var path: String
    var has_pwd: Bool
}
