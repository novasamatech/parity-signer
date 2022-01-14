//
//  MNetworkMenu.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 9.12.2021.
//

import Foundation

struct MNetworkMenu: Decodable, Hashable {
    var networks: [Network]
}

struct Network: Codable, Hashable {
    var key: String
    var logo: String
    var order: Int
    var selected: Bool
    var title: String
}
