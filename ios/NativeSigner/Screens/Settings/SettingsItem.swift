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
    case backupKeys
    case privacyPolicy
    case termsAndConditions
    case leaveFeedback
    case appVersion
    case wipe
}

extension SettingsItem {
    var title: String {
        switch self {
        case .networks:
            return Localizable.Settings.Label.networks.string
        case .verifier:
            return Localizable.Settings.Label.verifier.string
        case .backupKeys:
            return Localizable.Settings.Label.backup.string
        case .privacyPolicy:
            return Localizable.Settings.Label.policy.string
        case .termsAndConditions:
            return Localizable.Settings.Label.terms.string
        case .leaveFeedback:
            return Localizable.Settings.Label.feedback.string
        case .appVersion:
            return Localizable.Settings.Label.version.string
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

    var detailsNavigation: Action? {
        switch self {
        case .networks:
            return .manageNetworks
        case .verifier:
            return .viewGeneralVerifier
        case .backupKeys:
            return .backupSeed
        case .privacyPolicy:
            return .showDocuments
        case .termsAndConditions:
            return .showDocuments
        case .leaveFeedback:
            return nil
        case .appVersion:
            return nil
        case .wipe:
            return nil
        }
    }
}
