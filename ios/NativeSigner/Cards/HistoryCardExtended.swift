//
//  HistoryCardExtended.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 28.10.2021.
//

import SwiftUI

struct HistoryCardExtended: View {
    var event: MEventMaybeDecoded
    let timestamp = ""
    var body: some View {
        // swiftlint:disable:next closure_body_length
        HStack {
            switch event.event {
            case .databaseInitiated:
                HistoryCardTemplate(
                    image: .init(.iphoneArrow, variant: .forward),
                    danger: false,
                    line1: "Database initiated",
                    line2: ""
                )
            case .deviceWasOnline:
                HistoryCardTemplate(
                    image: .init(.xmark, variants: [.shield, .fill]),
                    danger: true,
                    line1: "Device was connected to network",
                    line2: ""
                )
            case let .generalVerifierSet(value):
                HistoryCardTemplate(
                    image: .init(.checkmark, variant: .shield),
                    danger: false,
                    line1: "General verifier set",
                    line2: value.show()
                )
            case .historyCleared:
                HistoryCardTemplate(
                    image: .init(.xmark, variants: [.rectangle, .portrait]),
                    danger: false,
                    line1: "History cleared",
                    line2: ""
                )
            case .identitiesWiped:
                HistoryCardTemplate(
                    image: .init(.xmark, variants: [.rectangle, .portrait]),
                    danger: false,
                    line1: "All keys were wiped",
                    line2: ""
                )
            case let .identityAdded(value):
                HistoryCardTemplate(
                    image: .init(.aqi, variant: .medium),
                    danger: false,
                    line1: "Key created",
                    line2: value.seedName + value.path + " in network with hash " +
                        value.networkGenesisHash.map { String(format: "%02X", $0) }.joined()
                )
            case let .identityRemoved(value):
                HistoryCardTemplate(
                    image: .init(.xmark, variants: [.rectangle, .portrait]),
                    danger: false,
                    line1: "Key removed",
                    line2: value.seedName + value.path + " in network with hash " +
                        value.networkGenesisHash.map { String(format: "%02X", $0) }.joined()
                )
            case let .metadataAdded(value):
                HistoryCardTemplate(
                    image: .init(.plus, variant: .viewfinder),
                    danger: false,
                    line1: "Metadata added",
                    line2: value.name + " version " + String(value.version)
                )
            case let .metadataRemoved(value):
                HistoryCardTemplate(
                    image: .init(.xmark, variants: [.rectangle, .portrait]),
                    danger: false,
                    line1: "Metadata removed",
                    line2: value.name + " version " + String(value.version)
                )
            case let .networkSpecsAdded(value):
                HistoryCardTemplate(
                    image: .init(.plus, variant: .viewfinder),
                    danger: false,
                    line1: "Network added",
                    line2: value.specs.title
                )
            case let .networkSpecsRemoved(value):
                HistoryCardTemplate(
                    image: .init(.xmark, variants: [.rectangle, .portrait]),
                    danger: false,
                    line1: "Network removed",
                    line2: value.specs.title
                )
            case let .networkVerifierSet(value):
                switch value.validCurrentVerifier {
                case .general:
                    HistoryCardTemplate(
                        image: .init(.checkmark, variant: .shield),
                        danger: false,
                        line1: "Network verifier set",
                        line2: value.generalVerifier.show() + " for network with genesis hash " +
                            value.genesisHash.map { String(format: "%02X", $0) }.joined()
                    )
                case let .custom(verifier):
                    HistoryCardTemplate(
                        image: .init(.checkmark, variant: .shield),
                        danger: false,
                        line1: "Network verifier set",
                        line2: verifier.show() + " for network with genesis hash " +
                            value.genesisHash.map { String(format: "%02X", $0) }.joined()
                    )
                }
            case .resetDangerRecord:
                HistoryCardTemplate(
                    image: .init(.checkmark, variant: .shield),
                    danger: true,
                    line1: "Warnings acknowledged",
                    line2: ""
                )
            case let .seedCreated(text):
                HistoryCardTemplate(
                    image: .init(.aqi, variant: .medium),
                    danger: false,
                    line1: "Seed created",
                    line2: text
                )
            case let .seedRemoved(text):
                HistoryCardTemplate(
                    image: .init(.xmark, variants: [.rectangle, .portrait, .fill]),
                    danger: false,
                    line1: "Seed removed",
                    line2: text
                )
            case let .seedNameWasShown(text):
                HistoryCardTemplate(
                    image: .init(.eye, variants: [.trianglebadge, .exclamationmark, .fill]),
                    danger: false,
                    line1: "Seed was shown",
                    line2: text
                )
            case let .networkSpecsSigned(value):
                HistoryCardTemplate(
                    image: .init(.signature),
                    danger: false,
                    line1: "Network specs signed",
                    line2: value.specsToSend.title
                )
            case let .metadataSigned(value):
                HistoryCardTemplate(
                    image: .init(.signature),
                    danger: false,
                    line1: "Metadata signed",
                    line2: value.name + String(value.version)
                )
            case let .typesSigned(value):
                HistoryCardTemplate(
                    image: .init(.signature),
                    danger: false,
                    line1: "Types signed",
                    line2: value.typesHash.map { String(format: "%02X", $0) }.joined()
                )
            case let .systemEntry(text):
                HistoryCardTemplate(
                    image: .init(.eye, variants: [.trianglebadge, .exclamationmark, .fill]),
                    danger: false,
                    line1: "System record",
                    line2: text
                )
            case let .transactionSignError(value):
                VStack {
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
            case .typesAdded:
                HistoryCardTemplate(
                    image: .init(.plus, variant: .viewfinder),
                    danger: false,
                    line1: "New types info loaded",
                    line2: ""
                )
            case .typesRemoved:
                HistoryCardTemplate(
                    image: .init(.minus, variant: .square),
                    danger: true,
                    line1: "Types info removed",
                    line2: ""
                )
            case let .userEntry(text):
                HistoryCardTemplate(
                    image: .init(.square),
                    danger: false,
                    line1: "User record",
                    line2: text
                )
            case let .warning(text):
                HistoryCardTemplate(
                    image: .init(.exclamationmark, variants: [.triangle, .fill]),
                    danger: true,
                    line1: "Warning! " + text,
                    line2: ""
                )
            case .wrongPassword:
                HistoryCardTemplate(
                    image: .init(.exclamationmark, variants: [.triangle, .fill]),
                    danger: true,
                    line1: "Wrong password entered",
                    line2: "operation was declined"
                )
            case let .messageSignError(value):
                HistoryCardTemplate(
                    image: .init(.exclamationmark, variants: [.triangle, .fill]),
                    danger: true,
                    line1: "Message signing error!",
                    line2: value.userComment
                )
            case let .messageSigned(value):
                HistoryCardTemplate(
                    image: .init(.signature),
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
