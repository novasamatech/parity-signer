//
//  Tab.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 17/08/2022.
//

import Foundation

/// Defines available tabs within main `Polkadot Vault` navigation
enum Tab: Equatable, Hashable {
    case keys
    case scanner
    case settings
}

extension Tab {
    var action: Action? {
        switch self {
        case .keys:
            return .navbarKeys
        case .scanner:
            return nil
        case .settings:
            return .navbarSettings
        }
    }
}
