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
                line1: "Database initiated"
            )
        case .deviceWasOnline:
            HistoryCardTemplate(
                image: .init(.xmark, variants: [.shield, .fill]),
                timestamp: timestamp,
                danger: true,
                line1: "Device was connected to network"
            )
        case let .generalVerifierSet(value):
            HistoryCardTemplate(
                image: .init(.checkmark, variant: .shield),
                timestamp: timestamp,
                danger: false,
                line1: "General verifier set",
                line2: value.show()
            )
        case .historyCleared:
            HistoryCardTemplate(
                image: .init(.xmark, variants: [.rectangle, .portrait]),
                timestamp: timestamp,
                danger: false,
                line1: "History cleared"
            )
        case .identitiesWiped:
            HistoryCardTemplate(
                image: .init(.xmark, variants: [.rectangle, .portrait]),
                timestamp: timestamp,
                danger: false,
                line1: "All keys were wiped"
            )
        case let .identityAdded(value):
            HistoryCardTemplate(
                image: .init(.aqi, variant: .medium),
                timestamp: timestamp,
                danger: false,
                line1: "Key created",
                line2: value.seedName + value.path
            )
        case let .identityRemoved(value):
            HistoryCardTemplate(
                image: .init(.xmark, variants: [.rectangle, .portrait]),
                timestamp: timestamp,
                danger: false,
                line1: "Key removed",
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
                line1: "Metadata added",
                line2: value.name + " version " + String(value.version)
            )
        case let .metadataRemoved(value):
            HistoryCardTemplate(
                image: .init(.xmark, variants: [.rectangle, .portrait]),
                timestamp: timestamp,
                danger: false,
                line1: "Metadata removed",
                line2: value.name + " version " + String(value.version)
            )
        case let .networkSpecsAdded(value):
            HistoryCardTemplate(
                image: .init(.plus, variant: .viewfinder),
                timestamp: timestamp,
                danger: false,
                line1: "Network added",
                line2: value.specs.title
            )
        case let .networkSpecsRemoved(value):
            HistoryCardTemplate(
                image: .init(.xmark, variants: [.rectangle, .portrait]),
                timestamp: timestamp,
                danger: false,
                line1: "Network removed",
                line2: value.specs.title
            )
        case let .networkVerifierSet(value):
            HistoryCardTemplate(
                image: .init(.checkmark, variant: .shield),
                timestamp: timestamp,
                danger: false,
                line1: "Network verifier set",
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
                line1: "Warnings acknowledged"
            )
        case let .seedCreated(text):
            HistoryCardTemplate(
                image: .init(.aqi, variant: .medium),
                timestamp: timestamp,
                danger: false,
                line1: "Seed created",
                line2: text
            )
        case let .seedRemoved(text):
            HistoryCardTemplate(
                image: .init(.xmark, variants: [.rectangle, .portrait, .fill]),
                timestamp: timestamp,
                danger: false,
                line1: "Seed removed",
                line2: text
            )
        case let .seedNameWasShown(text):
            HistoryCardTemplate(
                image: .init(.eye, variants: [.trianglebadge, .exclamationmark, .fill]),
                timestamp: timestamp,
                danger: false,
                line1: "Seed was shown",
                line2: text
            )
        case let .networkSpecsSigned(value):
            HistoryCardTemplate(
                image: .init(.signature),
                timestamp: timestamp,
                danger: false,
                line1: "Network specs signed",
                line2: value.specsToSend.title
            )
        case let .metadataSigned(value):
            HistoryCardTemplate(
                image: .init(.signature),
                timestamp: timestamp,
                danger: false,
                line1: "Metadata signed",
                line2: value.name + String(value.version)
            )
        case .typesSigned:
            HistoryCardTemplate(
                image: .init(.signature),
                timestamp: timestamp,
                danger: false,
                line1: "Types signed"
            )
        case let .systemEntry(text):
            HistoryCardTemplate(
                image: .init(.eye, variants: [.trianglebadge, .exclamationmark, .fill]),
                timestamp: timestamp,
                danger: false,
                line1: "System record",
                line2: text
            )
        case let .transactionSignError(value):
            HistoryCardTemplate(
                image: .init(.exclamationmark, variants: [.triangle, .fill]),
                timestamp: timestamp,
                danger: true,
                line1: "Signing failure",
                line2: value.userComment
            )
        case let .transactionSigned(value):
            HistoryCardTemplate(
                image: .init(.signature),
                timestamp: timestamp,
                danger: false,
                line1: "Generated signature",
                line2: value.userComment
            )
        case .typesAdded:
            HistoryCardTemplate(
                image: .init(.plus, variant: .viewfinder),
                timestamp: timestamp,
                danger: false,
                line1: "New types info loaded"
            )
        case .typesRemoved:
            HistoryCardTemplate(
                image: .init(.minus, variant: .square),
                timestamp: timestamp,
                danger: true,
                line1: "Types info removed"
            )
        case let .userEntry(text):
            HistoryCardTemplate(
                image: .init(.square),
                timestamp: timestamp,
                danger: false,
                line1: "User record",
                line2: text
            )
        case let .warning(text):
            HistoryCardTemplate(
                image: .init(.exclamationmark, variants: [.triangle, .fill]),
                timestamp: timestamp,
                danger: true,
                line1: "Warning! " + text
            )
        case .wrongPassword:
            HistoryCardTemplate(
                image: .init(.exclamationmark, variants: [.triangle, .fill]),
                timestamp: timestamp,
                danger: true,
                line1: "Wrong password entered",
                line2: "operation was declined"
            )
        case let .messageSignError(value):
            HistoryCardTemplate(
                image: .init(.exclamationmark, variants: [.triangle, .fill]),
                timestamp: timestamp,
                danger: true,
                line1: "Message signing error!",
                line2: value.userComment
            )
        case let .messageSigned(value):
            HistoryCardTemplate(
                image: .init(.signature),
                timestamp: timestamp,
                danger: false,
                line1: "Generated signature for message",
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
