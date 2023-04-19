//
//  SettingsItem.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 12/12/2022.
//

import Foundation

enum SettingsItem: Equatable, Hashable, CaseIterable {
    case logs
    case networks
    case verifier
    case backup
    case privacyPolicy
    case termsAndConditions
    case wipe
}

extension SettingsItem {
    var title: String {
        switch self {
        case .logs:
            return Localizable.Settings.Label.logs.string
        case .networks:
            return Localizable.Settings.Label.networks.string
        case .verifier:
            return Localizable.Settings.Label.verifier.string
        case .backup:
            return Localizable.Settings.Label.backup.string
        case .privacyPolicy:
            return Localizable.Settings.Label.policy.string
        case .termsAndConditions:
            return Localizable.Settings.Label.terms.string
        case .wipe:
            return Localizable.Settings.Label.wipe.string
        }
    }

    var isDestructive: Bool {
        [.wipe].contains(self)
    }

    var hasDetails: Bool {
        ![.wipe].contains(self)
    }

    var nativeNavigation: Bool {
        ![.wipe, .networks, .verifier].contains(self)
    }
}
