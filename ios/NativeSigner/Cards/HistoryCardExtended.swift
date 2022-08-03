//
//  HistoryCardExtended.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 28.10.2021.
//

import SwiftUI

// swiftlint:disable:next type_body_length
struct HistoryCardExtended: View {
    var event: MEventMaybeDecoded
    let timestamp = ""
    var body: some View {
        HStack {
            switch event.event {
            case .databaseInitiated: HistoryCardTemplate(
                    image: "iphone.and.arrow.forward",
                    timestamp: timestamp,
                    danger: false,
                    line1: "Database initiated",
                    line2: ""
                )
            case .deviceWasOnline: HistoryCardTemplate(
                    image: "xmark.shield.fill",
                    timestamp: timestamp,
                    danger: true,
                    line1: "Device was connected to network",
                    line2: ""
                )
            case let .generalVerifierSet(value): HistoryCardTemplate(
                    image: "checkmark.shield",
                    timestamp: timestamp,
                    danger: false,
                    line1: "General verifier set",
                    line2: value.show()
                )
            case .historyCleared: HistoryCardTemplate(
                    image: "xmark.rectangle.portrait",
                    timestamp: timestamp,
                    danger: false,
                    line1: "History cleared",
                    line2: ""
                )
            case .identitiesWiped: HistoryCardTemplate(
                    image: "xmark.rectangle.portrait",
                    timestamp: timestamp,
                    danger: false,
                    line1: "All keys were wiped",
                    line2: ""
                )
            case let .identityAdded(value): HistoryCardTemplate(
                    image: "aqi.medium",
                    timestamp: timestamp,
                    danger: false,
                    line1: "Key created",
                    line2: value.seedName + value.path + " in network with hash " +
                        value.networkGenesisHash.map { String(format: "%02X", $0) }.joined()
                )
            case let .identityRemoved(value): HistoryCardTemplate(
                    image: "xmark.rectangle.portrait",
                    timestamp: timestamp,
                    danger: false,
                    line1: "Key removed",
                    line2: value.seedName + value.path + " in network with hash " +
                        value.networkGenesisHash.map { String(format: "%02X", $0) }.joined()
                )
            case let .secretWasExported(value): HistoryCardTemplate(
                    image: "eye.trianglebadge.exclamationmark",
                    timestamp: timestamp,
                    danger: true,
                    line1: "Secret was exported",
                    line2: value.seedName + value.path + " in network with hash " +
                            value.networkGenesisHash.map { String(format: "%02X", $0) }.joined()
            )
            case let .metadataAdded(value): HistoryCardTemplate(
                    image: "plus.viewfinder",
                    timestamp: timestamp,
                    danger: false,
                    line1: "Metadata added",
                    line2: value.name + " version " + String(value.version)
                )
            case let .metadataRemoved(value): HistoryCardTemplate(
                    image: "xmark.rectangle.portrait",
                    timestamp: timestamp,
                    danger: false,
                    line1: "Metadata removed",
                    line2: value.name + " version " + String(value.version)
                )
            case let .networkSpecsAdded(value): HistoryCardTemplate(
                    image: "plus.viewfinder",
                    timestamp: timestamp,
                    danger: false,
                    line1: "Network added",
                    line2: value.specs.title
                )
            case let .networkSpecsRemoved(value): HistoryCardTemplate(
                    image: "xmark.rectangle.portrait",
                    timestamp: timestamp,
                    danger: false,
                    line1: "Network removed",
                    line2: value.specs.title
                )
            case let .networkVerifierSet(value):
                switch value.validCurrentVerifier {
                case .general:
                    HistoryCardTemplate(
                        image: "checkmark.shield",
                        timestamp: timestamp,
                        danger: false,
                        line1: "Network verifier set",
                        line2: value.generalVerifier.show() + " for network with genesis hash " +
                            value.genesisHash.map { String(format: "%02X", $0) }.joined()
                    )
                case let .custom(verifier):
                    HistoryCardTemplate(
                        image: "checkmark.shield",
                        timestamp: timestamp,
                        danger: false,
                        line1: "Network verifier set",
                        line2: verifier.show() + " for network with genesis hash " +
                            value.genesisHash.map { String(format: "%02X", $0) }.joined()
                    )
                }
            case .resetDangerRecord: HistoryCardTemplate(
                    image: "checkmark.shield",
                    timestamp: timestamp,
                    danger: true,
                    line1: "Warnings acknowledged",
                    line2: ""
                )
            case let .seedCreated(text):
                HistoryCardTemplate(
                    image: "aqi.medium",
                    timestamp: timestamp,
                    danger: false,
                    line1: "Seed created",
                    line2: text
                )
            case let .seedRemoved(text): HistoryCardTemplate(
                    image: "xmark.rectangle.portrait.fill",
                    timestamp: timestamp,
                    danger: false,
                    line1: "Seed removed",
                    line2: text
                )
            case let .seedNameWasShown(text): HistoryCardTemplate(
                    image: "eye.trianglebadge.exclamationmark.fill",
                    timestamp: timestamp,
                    danger: false,
                    line1: "Seed was shown",
                    line2: text
                )
            case let .networkSpecsSigned(value): HistoryCardTemplate(
                    image: "signature",
                    timestamp: timestamp,
                    danger: false,
                    line1: "Network specs signed",
                    line2: value.specsToSend.title
                )
            case let .metadataSigned(value): HistoryCardTemplate(
                    image: "signature",
                    timestamp: timestamp,
                    danger: false,
                    line1: "Metadata signed",
                    line2: value.name + String(value.version)
                )
            case let .typesSigned(value): HistoryCardTemplate(
                    image: "signature",
                    timestamp: timestamp,
                    danger: false,
                    line1: "Types signed",
                    line2: value.typesHash.map { String(format: "%02X", $0) }.joined()
                )
            case let .systemEntry(text): HistoryCardTemplate(
                    image: "eye.trianglebadge.exclamationmark.fill",
                    timestamp: timestamp,
                    danger: false,
                    line1: "System record",
                    line2: text
                )
            case let .transactionSignError(value): VStack {
                    Text("Transaction failed")
                    TransactionBlock(cards: event.decoded?.assemble() ?? [])
                    Text("Signed by: ")
                    HStack {
                        Identicon(identicon: event.signedBy?.identicon ?? [])
                        VStack {
                            Text(value.signedBy.show())
                            Text((event.signedBy?.seedName ?? "") + (event.signedBy?.path ?? ""))
                        }
                    }
                    Text("in network")
                    Text(value.networkName)
                    Text("Comment :")
                    Text(value.userComment)
                }
            case let .transactionSigned(value):
                VStack {
                    TransactionBlock(cards: event.decoded?.assemble() ?? [])
                    Text("Signed by: ")
                    HStack {
                        Identicon(identicon: event.signedBy?.identicon ?? [])
                        VStack {
                            Text(value.signedBy.show())
                            Text((event.signedBy?.seedName ?? "") + (event.signedBy?.path ?? ""))
                        }
                    }
                    Text("in network")
                    Text(value.networkName)
                    Text("Comment :")
                    Text(value.userComment)
                }
            case .typesAdded: HistoryCardTemplate(
                    image: "plus.viewfinder",
                    timestamp: timestamp,
                    danger: false,
                    line1: "New types info loaded",
                    line2: ""
                )
            case .typesRemoved: HistoryCardTemplate(
                    image: "minus.square",
                    timestamp: timestamp,
                    danger: true,
                    line1: "Types info removed",
                    line2: ""
                )
            case let .userEntry(text): HistoryCardTemplate(
                    image: "square",
                    timestamp: timestamp,
                    danger: false,
                    line1: "User record",
                    line2: text
                )
            case let .warning(text): HistoryCardTemplate(
                    image: "exclamationmark.triangle.fill",
                    timestamp: timestamp,
                    danger: true,
                    line1: "Warning! " + text,
                    line2: ""
                )
            case .wrongPassword: HistoryCardTemplate(
                    image: "exclamationmark.triangle.fill",
                    timestamp: timestamp,
                    danger: true,
                    line1: "Wrong password entered",
                    line2: "operation was declined"
                )
            case let .messageSignError(value): HistoryCardTemplate(
                    image: "exclamationmark.triangle.fill",
                    timestamp: timestamp,
                    danger: true,
                    line1: "Message signing error!",
                    line2: value.userComment
                )
            case let .messageSigned(value): HistoryCardTemplate(
                    image: "signature",
                    timestamp: timestamp,
                    danger: false,
                    line1: "Generated signature for message",
                    line2: value.userComment
                )
            }
        }
    }
}

// struct HistoryCardExtended_Previews: PreviewProvider {
// static var previews: some View {
// HistoryCardExtended()
// }
// }
