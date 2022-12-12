//
//  Tab.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 17/08/2022.
//

import Foundation

/// Defines available tabs within main Signer navigation
enum Tab: CaseIterable, Equatable {
    case keys
    case scanner
    case logs
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
        case .log:
            self = .logs
        case .scan:
            self = .scanner
        case .keys:
            self = .keys
        case .settings:
            self = .settings
        case .back:
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
        case .logs:
            return .navbarLog
        case .settings:
            return .navbarSettings
        }
    }
}
