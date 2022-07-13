//
//  HistoryCard.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 13.8.2021.
//

import SwiftUI

struct HistoryCard: View {
    var event: Event
    var timestamp: String
    var body: some View {
        HStack {
            switch event {
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
            case .generalVerifierSet(let value): HistoryCardTemplate(
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
            case .identityAdded(let value): HistoryCardTemplate(
                image: "aqi.medium",
                timestamp: timestamp,
                danger: false,
                line1: "Key created",
                line2: value.seedName + value.path
            )
            case .identityRemoved(let value): HistoryCardTemplate(
                image: "xmark.rectangle.portrait",
                timestamp: timestamp,
                danger: false,
                line1: "Key removed",
                line2: value.seedName + value.path
            )
            case .metadataAdded(let value): HistoryCardTemplate(
                image: "plus.viewfinder",
                timestamp: timestamp,
                danger: false,
                line1: "Metadata added",
                line2: value.name + " version " +  String(value.version)
            )
            case .metadataRemoved(let value): HistoryCardTemplate(
                image: "xmark.rectangle.portrait",
                timestamp: timestamp,
                danger: false,
                line1: "Metadata removed",
                line2: value.name + " version " +  String(value.version)
            )
            case .networkSpecsAdded(let value): HistoryCardTemplate(
                image: "plus.viewfinder",
                timestamp: timestamp,
                danger: false,
                line1: "Network added",
                line2: value.specs.title
            )
            case .networkSpecsRemoved(let value): HistoryCardTemplate(
                image: "xmark.rectangle.portrait",
                timestamp: timestamp,
                danger: false,
                line1: "Network removed",
                line2: value.specs.title
            )
            case .networkVerifierSet(let value): HistoryCardTemplate(
                image: "checkmark.shield",
                timestamp: timestamp,
                danger: false,
                line1: "Network verifier set",
                line2: value.validCurrentVerifier == .general ?
                "general" :
                    "custom" + " for network with genesis hash " +
                value.genesisHash.map {String(format: "%02X", $0)}.joined()
            )
            case .resetDangerRecord: HistoryCardTemplate(
                image: "checkmark.shield",
                timestamp: timestamp,
                danger: true,
                line1: "Warnings acknowledged",
                line2: ""
            )
            case .seedCreated(let text):
                HistoryCardTemplate(
                    image: "aqi.medium",
                    timestamp: timestamp,
                    danger: false,
                    line1: "Seed created",
                    line2: text
                )
            case .seedNameWasShown(let text): HistoryCardTemplate(
                image: "eye.trianglebadge.exclamationmark.fill",
                timestamp: timestamp,
                danger: false,
                line1: "Seed was shown",
                line2: text
            )
            case .networkSpecsSigned(let value): HistoryCardTemplate(
                image: "signature",
                timestamp: timestamp,
                danger: false,
                line1: "Network specs signed",
                line2: value.specsToSend.title
            )
            case .metadataSigned(let value): HistoryCardTemplate(
                image: "signature",
                timestamp: timestamp,
                danger: false,
                line1: "Metadata signed",
                line2: value.name + String(value.version)
            )
            case .typesSigned: HistoryCardTemplate(
                image: "signature",
                timestamp: timestamp,
                danger: false,
                line1: "Types signed",
                line2: ""
            )
            case .systemEntry(let text): HistoryCardTemplate(
                image: "eye.trianglebadge.exclamationmark.fill",
                timestamp: timestamp,
                danger: false,
                line1: "System record",
                line2: text
            )
            case .transactionSignError(let value): HistoryCardTemplate(
                image: "exclamationmark.triangle.fill",
                timestamp: timestamp,
                danger: true,
                line1: "Signing failure",
                line2: value.userComment
            )
            case .transactionSigned(let value): HistoryCardTemplate(
                image: "signature",
                timestamp: timestamp,
                danger: false,
                line1: "Generated signature",
                line2: value.userComment
            )
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
            case .userEntry(let text): HistoryCardTemplate(
                image: "square",
                timestamp: timestamp,
                danger: false,
                line1: "User record",
                line2: text
            )
            case .warning(let text): HistoryCardTemplate(
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
            case .messageSignError(let value): HistoryCardTemplate(
                image: "exclamationmark.triangle.fill",
                timestamp: timestamp,
                danger: true,
                line1: "Message signing error!",
                line2: value.userComment
            )
            case .messageSigned(let value): HistoryCardTemplate(
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

/*
 struct HistoryCard_Previews: PreviewProvider {
 static var previews: some View {
 HistoryCard()
 }
 }
 */
