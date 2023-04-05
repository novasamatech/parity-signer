//
//  Tab.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 17/08/2022.
//

import Foundation

/// Defines available tabs within main `Polkadot Vault` navigation
enum Tab: CaseIterable, Equatable {
    case keys
    case scanner
    case settings
}

extension Tab {
    /// Initialise `Tab` based on `ActionResult` subcomponent returned from Rust navigation system
    /// - Parameter footerButton: value from Rust backend domain that informs which tab is selected.
    /// `FooterButton` also contains `back` which is not relevant to `Tab`
    init?(_ footerButton: FooterButton?) {
        guard let footerButton = footerButton else {
            return nil
        }
        switch footerButton {
        case .scan:
            self = .scanner
        case .keys:
            self = .keys
        case .settings:
            self = .settings
        case .back,
             .log:
            return nil
        }
    }
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
