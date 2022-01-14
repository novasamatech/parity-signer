//
//  MManageNetworks.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 20.12.2021.
//

import Foundation

struct MManageNetworks: Decodable {
    var networks: [MMNetwork]
}

struct MMNetwork: Decodable {
    var key: String
    var title: String
    var logo: String
    var order: Int
}
