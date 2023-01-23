//
//  SettingsItem.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 12/12/2022.
//

import Foundation

enum SettingsItem: Equatable, CaseIterable {
    case networks
    case verifier
    case privacyPolicy
    case termsAndConditions
    case wipe
}

extension SettingsItem {
    var title: String {
        switch self {
        case .networks:
            return Localizable.Settings.Label.networks.string
        case .verifier:
            return Localizable.Settings.Label.verifier.string
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
}
