//
//  UInt8+Formatting.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 04/08/2022.
//

import Foundation

extension [UInt8] {
    /// Utility formatter to parse `[UInt8]` into UI ready `String`
    var formattedAsString: String {
        map { String(format: "%02X", $0) }.joined()
    }
}
