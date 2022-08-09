//
//  HistoryCard.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 13.8.2021.
//

import SwiftUI

struct HistoryCard: View {
    var event: Event
    var timestamp: String?
    var body: some View {
        switch event {
        case .databaseInitiated:
            HistoryCardTemplate(
                image: .init(.iphoneArrow, variant: .forward),
                timestamp: timestamp,
                danger: false,
                line1: Localizable.HistoryCard.database.string
            )
        case .deviceWasOnline:
            HistoryCardTemplate(
                image: .init(.xmark, variants: [.shield, .fill]),
                timestamp: timestamp,
                danger: true,
                line1: Localizable.HistoryCard.deviceConnected.string
            )
        case let .generalVerifierSet(value):
            HistoryCardTemplate(
                image: .init(.checkmark, variant: .shield),
                timestamp: timestamp,
                danger: false,
                line1: Localizable.HistoryCard.verifierSet.string,
                line2: value.show()
            )
        case .historyCleared:
            HistoryCardTemplate(
                image: .init(.xmark, variants: [.rectangle, .portrait]),
                timestamp: timestamp,
                danger: false,
                line1: Localizable.HistoryCard.historyCleared.string
            )
        case .identitiesWiped:
            HistoryCardTemplate(
                image: .init(.xmark, variants: [.rectangle, .portrait]),
                timestamp: timestamp,
                danger: false,
                line1: Localizable.HistoryCard.keysCleared.string
            )
        case let .identityAdded(value):
            HistoryCardTemplate(
                image: .init(.aqi, variant: .medium),
                timestamp: timestamp,
                danger: false,
                line1: Localizable.HistoryCard.keysCreated.string,
                line2: value.seedName + value.path
            )
        case let .identityRemoved(value):
            HistoryCardTemplate(
                image: .init(.xmark, variants: [.rectangle, .portrait]),
                timestamp: timestamp,
                danger: false,
                line1: Localizable.HistoryCard.keysRemoved.string,
                line2: value.seedName + value.path
            )
        case let .secretWasExported(value):
            HistoryCardTemplate(
                image: .init(.eye, variants: [.trianglebadge, .exclamationmark, .fill]),
                timestamp: timestamp,
                danger: true,
                line1: "Secret was exported",
                line2: value.seedName + value.path
            )
        case let .metadataAdded(value):
            HistoryCardTemplate(
                image: .init(.plus, variant: .viewfinder),
                timestamp: timestamp,
                danger: false,
                line1: Localizable.HistoryCard.metadataAdded.string,
                line2: value.name + " version " + String(value.version)
            )
        case let .metadataRemoved(value):
            HistoryCardTemplate(
                image: .init(.xmark, variants: [.rectangle, .portrait]),
                timestamp: timestamp,
                danger: false,
                line1: Localizable.HistoryCard.metadataRemoved.string,

                line2: value.name + " version " + String(value.version)
            )
        case let .networkSpecsAdded(value):
            HistoryCardTemplate(
                image: .init(.plus, variant: .viewfinder),
                timestamp: timestamp,
                danger: false,
                line1: Localizable.HistoryCard.networkAdded.string,
                line2: value.specs.title
            )
        case let .networkSpecsRemoved(value):
            HistoryCardTemplate(
                image: .init(.xmark, variants: [.rectangle, .portrait]),
                timestamp: timestamp,
                danger: false,
                line1: Localizable.HistoryCard.networkRemoved.string,
                line2: value.specs.title
            )
        case let .networkVerifierSet(value):
            HistoryCardTemplate(
                image: .init(.checkmark, variant: .shield),
                timestamp: timestamp,
                danger: false,
                line1: Localizable.HistoryCard.networkVerifier.string,
                line2: value.validCurrentVerifier == .general ?
                    "general" :
                    "custom" + " for network with genesis hash " +
                    value.genesisHash.formattedAsString
            )
        case .resetDangerRecord:
            HistoryCardTemplate(
                image: .init(.checkmark, variant: .shield),
                timestamp: timestamp,
                danger: true,
                line1: Localizable.HistoryCard.resetDanger.string
            )
        case let .seedCreated(text):
            HistoryCardTemplate(
                image: .init(.aqi, variant: .medium),
                timestamp: timestamp,
                danger: false,
                line1: Localizable.HistoryCard.seedCreated.string,
                line2: text
            )
        case let .seedRemoved(text):
            HistoryCardTemplate(
                image: .init(.xmark, variants: [.rectangle, .portrait, .fill]),
                timestamp: timestamp,
                danger: false,
                line1: Localizable.HistoryCard.seedRemoved.string,
                line2: text
            )
        case let .seedNameWasShown(text):
            HistoryCardTemplate(
                image: .init(.eye, variants: [.trianglebadge, .exclamationmark, .fill]),
                timestamp: timestamp,
                danger: false,
                line1: Localizable.HistoryCard.seedShown.string,
                line2: text
            )
        case let .networkSpecsSigned(value):
            HistoryCardTemplate(
                image: .init(.signature),
                timestamp: timestamp,
                danger: false,
                line1: Localizable.HistoryCard.networkSpecsSigned.string,
                line2: value.specsToSend.title
            )
        case let .metadataSigned(value):
            HistoryCardTemplate(
                image: .init(.signature),
                timestamp: timestamp,
                danger: false,
                line1: Localizable.HistoryCard.metadataSigned.string,
                line2: value.name + String(value.version)
            )
        case .typesSigned:
            HistoryCardTemplate(
                image: .init(.signature),
                timestamp: timestamp,
                danger: false,
                line1: Localizable.HistoryCard.typesSigned.string
            )
        case let .systemEntry(text):
            HistoryCardTemplate(
                image: .init(.eye, variants: [.trianglebadge, .exclamationmark, .fill]),
                timestamp: timestamp,
                danger: false,
                line1: Localizable.HistoryCard.systemRecord.string,
                line2: text
            )
        case let .transactionSignError(value):
            HistoryCardTemplate(
                image: .init(.exclamationmark, variants: [.triangle, .fill]),
                timestamp: timestamp,
                danger: true,
                line1: Localizable.HistoryCard.signingFailure.string,
                line2: value.userComment
            )
        case let .transactionSigned(value):
            HistoryCardTemplate(
                image: .init(.signature),
                timestamp: timestamp,
                danger: false,
                line1: Localizable.HistoryCard.transactionSigned.string,
                line2: value.userComment
            )
        case .typesAdded:
            HistoryCardTemplate(
                image: .init(.plus, variant: .viewfinder),
                timestamp: timestamp,
                danger: false,
                line1: Localizable.HistoryCard.typesAdded.string
            )
        case .typesRemoved:
            HistoryCardTemplate(
                image: .init(.minus, variant: .square),
                timestamp: timestamp,
                danger: true,
                line1: Localizable.HistoryCard.typesRemoved.string
            )
        case let .userEntry(text):
            HistoryCardTemplate(
                image: .init(.square),
                timestamp: timestamp,
                danger: false,
                line1: Localizable.HistoryCard.userRecord.string,
                line2: text
            )
        case let .warning(text):
            HistoryCardTemplate(
                image: .init(.exclamationmark, variants: [.triangle, .fill]),
                timestamp: timestamp,
                danger: true,
                line1: Localizable.HistoryCard.warning(text)
            )
        case .wrongPassword:
            HistoryCardTemplate(
                image: .init(.exclamationmark, variants: [.triangle, .fill]),
                timestamp: timestamp,
                danger: true,
                line1: Localizable.HistoryCard.WrongPassword.title.string,
                line2: Localizable.HistoryCard.WrongPassword.subtitle.string
            )
        case let .messageSignError(value):
            HistoryCardTemplate(
                image: .init(.exclamationmark, variants: [.triangle, .fill]),
                timestamp: timestamp,
                danger: true,
                line1: Localizable.HistoryCard.messageSignError.string,
                line2: value.userComment
            )
        case let .messageSigned(value):
            HistoryCardTemplate(
                image: .init(.signature),
                timestamp: timestamp,
                danger: false,
                line1: Localizable.HistoryCard.messageSigned.string,
                line2: value.userComment
            )
        }
    }
}

// struct HistoryCard_Previews: PreviewProvider {
// static var previews: some View {
// HistoryCard()
// }
// }
