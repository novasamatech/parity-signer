//
//  ErrorDisplayed+TransactionSigning.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 09/02/2023.
//

import Foundation

extension ErrorDisplayed {
    var transactionError: TransactionError {
        switch self {
        case .DbNotInitialized:
            return .generic("")
        case let .LoadMetaUnknownNetwork(name):
            return .metadataForUnknownNetwork(name: name)
        case let .MetadataKnown(name, version):
            return .metadataAlreadyAdded(name: name, version: String(version))
        case .MutexPoisoned:
            return .generic("")
        case let .SpecsKnown(name, encryption):
            return .networkAlreadyAdded(name: name, encryption: encryption.rawValue)
        case let .UnknownNetwork(genesisHash, encryption):
            return .unknownNetwork(genesisHash: genesisHash.formattedAsString, encryption: encryption.rawValue)
        case let .MetadataOutdated(name, have, want):
            return .outdatedMetadata(name: name, currentVersion: String(have), expectedVersion: String(want))
        case let .Str(errorMessage):
            return .generic(errorMessage)
        }
    }
}

enum TransactionError: Error {
    case generic(String)
    case metadataForUnknownNetwork(name: String)
    case networkAlreadyAdded(name: String, encryption: String)
    case metadataAlreadyAdded(name: String, version: String)
    case outdatedMetadata(name: String, currentVersion: String, expectedVersion: String)
    case unknownNetwork(genesisHash: String, encryption: String)
}

// swiftlint:disable all
extension ErrorBottomModalViewModel {
    static func transactionError(for transactionError: TransactionError) -> ErrorBottomModalViewModel {
        switch transactionError {
        case let .generic(message):
            return ErrorBottomModalViewModel.alertError(message: message)
        case let .metadataForUnknownNetwork(name):
            fatalError()
        case let .networkAlreadyAdded(name, encryption):
            fatalError()
        case let .metadataAlreadyAdded(name, version):
            fatalError()
        case let .outdatedMetadata(name, currentVersion, expectedVersion):
            return ErrorBottomModalViewModel.signingInvalidNetworkVersion(name)
        case let .unknownNetwork(genesisHash, encryption):
            return ErrorBottomModalViewModel.signingUnknownNetwork()
        }
    }
}
