//
//  ErrorDisplayed+LocalizedDescription.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 05/06/2023.
//

import Foundation

extension ErrorDisplayed {
    var localizedDescription: String {
        switch self {
        case .DbNotInitialized:
            return Localizable.ErrorDisplayed.dbNotInitialized.string
        case let .LoadMetaUnknownNetwork(name):
            return Localizable.TransactionSign.Error.MetadataUnknownNetwork.message(name)
        case let .MetadataKnown(name, version):
            return Localizable.TransactionSign.Error.MetadataAlreadyAdded.title(name, version)
        case .MutexPoisoned:
            return Localizable.ErrorDisplayed.mutexPoisoned.string
        case let .SpecsKnown(name, _):
            return Localizable.TransactionSign.Error.NetworkAlreadyAdded.title(name)
        case .UnknownNetwork:
            return Localizable.TransactionSign.Error.UnknownNetwork.message.string
        case let .MetadataOutdated(name, have, want):
            return Localizable.TransactionSign.Error.OutdatedMetadata.message(name, String(have), String(want))
        case let .NoMetadata(name):
            return Localizable.TransactionSign.Error.NoMetadataForNetwork.message(name)
        case let .Str(errorMessage):
            return errorMessage
        }
    }
}
