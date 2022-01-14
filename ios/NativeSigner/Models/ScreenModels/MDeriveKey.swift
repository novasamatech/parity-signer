//
//  MDeriveKey.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 13.12.2021.
//

import Foundation

struct MDeriveKey: Decodable, Equatable {
    var seed_name: String
    var network_title: String
    var network_logo: String
    var suggested_derivation: String
    var keyboard: Bool
}
