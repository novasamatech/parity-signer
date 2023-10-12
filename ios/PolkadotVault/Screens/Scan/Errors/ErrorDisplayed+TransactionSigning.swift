//
//  ErrorDisplayed+TransactionSigning.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 09/02/2023.
//

import Foundation

extension ErrorDisplayed {
    var transactionError: TransactionError {
        switch self {
        case .DbNotInitialized:
            .generic("")
        case let .LoadMetaUnknownNetwork(name):
            .metadataForUnknownNetwork(name: name)
        case let .MetadataKnown(name, version):
            .metadataAlreadyAdded(name: name, version: String(version))
        case .MutexPoisoned:
            .generic("")
        case let .SpecsKnown(name, encryption):
            .networkAlreadyAdded(name: name, encryption: encryption.rawValue)
        case let .UnknownNetwork(genesisHash, encryption):
            .unknownNetwork(genesisHash: genesisHash.formattedAsString, encryption: encryption.rawValue)
        case let .MetadataOutdated(name, have, want):
            .outdatedMetadata(name: name, currentVersion: String(have), expectedVersion: String(want))
        case let .NoMetadata(name):
            .noMetadataForNetwork(name: name)
        case let .Str(errorMessage):
            .generic(errorMessage)
        case .WrongPassword:
            .generic(Localizable.ErrorDisplayed.wrongPassword.string)
        case .DbSchemaMismatch:
            .generic("")
        }
    }
}

enum TransactionError: Error, Equatable {
    case generic(String)
    case metadataForUnknownNetwork(name: String)
    case networkAlreadyAdded(name: String, encryption: String)
    case metadataAlreadyAdded(name: String, version: String)
    case outdatedMetadata(name: String, currentVersion: String, expectedVersion: String)
    case unknownNetwork(genesisHash: String, encryption: String)
    case noMetadataForNetwork(name: String)
}

// swiftlint:disable all
extension ErrorBottomModalViewModel {
    static func transactionError(for transactionError: TransactionError) -> ErrorBottomModalViewModel {
        switch transactionError {
        case let .generic(message):
            ErrorBottomModalViewModel.alertError(message: message)
        case let .metadataForUnknownNetwork(name):
            ErrorBottomModalViewModel.metadataForUnknownNetwork(name)
        case let .networkAlreadyAdded(name, _):
            ErrorBottomModalViewModel.networkAlreadyAdded(name)
        case let .metadataAlreadyAdded(name, version):
            ErrorBottomModalViewModel.metadataAlreadyAdded(name, version)
        case let .outdatedMetadata(name, currentVersion, expectedVersion):
            ErrorBottomModalViewModel.outdatedMetadata(name, currentVersion, expectedVersion)
        case .unknownNetwork:
            ErrorBottomModalViewModel.signingUnknownNetwork()
        case let .noMetadataForNetwork(name):
            ErrorBottomModalViewModel.noMetadataForNetwork(name)
        }
    }
}
