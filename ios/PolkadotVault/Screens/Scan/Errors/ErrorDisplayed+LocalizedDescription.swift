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
            Localizable.ErrorDisplayed.dbNotInitialized.string
        case let .LoadMetaUnknownNetwork(name):
            Localizable.TransactionSign.Error.MetadataUnknownNetwork.message(name)
        case let .MetadataKnown(name, version):
            Localizable.TransactionSign.Error.MetadataAlreadyAdded.title(name, version)
        case .MutexPoisoned:
            Localizable.ErrorDisplayed.mutexPoisoned.string
        case let .SpecsKnown(name, _):
            Localizable.TransactionSign.Error.NetworkAlreadyAdded.title(name)
        case .UnknownNetwork:
            Localizable.TransactionSign.Error.UnknownNetwork.message.string
        case let .MetadataOutdated(name, have, want):
            Localizable.TransactionSign.Error.OutdatedMetadata.message(name, String(have), String(want))
        case let .NoMetadata(name):
            Localizable.TransactionSign.Error.NoMetadataForNetwork.message(name)
        case let .Str(errorMessage):
            errorMessage
        case .WrongPassword:
            Localizable.ErrorDisplayed.wrongPassword.string
        case .DbSchemaMismatch:
            Localizable.ErrorDisplayed.dbSchemaMismatch.string
        }
    }
}
