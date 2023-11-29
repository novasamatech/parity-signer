//
//  SFSymbols.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 04/08/2022.
//

import SwiftUI

/// Base values for Apple's SF Symbols
///
/// Refer to https://developer.apple.com/sf-symbols/ for currently available combinations
enum SFSymbols: String {
    case lock
}

extension Image {
    init(_ symbol: SFSymbols) {
        self.init(systemName: symbol.rawValue)
    }
}
