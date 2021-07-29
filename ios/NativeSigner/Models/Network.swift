//
//  Network.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import SwiftUI

struct Network: Codable {
    var key: String
    var color: String
    var logo: String
    var order: String
    var secondaryColor: String
    var title: String
}

extension Network {
    static var networkData: [Network] {
        [
            Network(key: "111111", color: "0xFFFFFF", logo: "1", order: "0", secondaryColor: "0xFFFFFF", title: "Ololonet"),
            Network(key: "111111", color: "0xFFFFFF", logo: "2", order: "1", secondaryColor: "0xFFFFFF", title: "Pyshpysh"),
            Network(key: "111111", color: "0xFFFFFF", logo: "3", order: "2", secondaryColor: "0xFFFFFF", title: "Kekeke"),
            Network(key: "111111", color: "0xFFFFFF", logo: "4", order: "3", secondaryColor: "0xFFFFFF", title: "Kuskuskus")
        ]
    }
}
