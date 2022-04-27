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
                line2: ""//value.v?.publicKey?.truncateMiddle(length: 8) ?? "error" + "\n" + value.v?.encryption? ?? "error"
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
                line2: value.seedName.decode64() + value.path
            )
            case .identityRemoved(let value): HistoryCardTemplate(
                image: "xmark.rectangle.portrait",
                timestamp: timestamp,
                danger: false,
                line1: "Key removed",
                line2: value.seedName.decode64() + value.path
            )
            case .metadataAdded(let value): HistoryCardTemplate(
                image: "plus.viewfinder",
                timestamp: timestamp,
                danger: false,
                line1: "Metadata added",
                line2: ""//value.name + " version " +  value.version
            )
            case .metadataRemoved(let value): HistoryCardTemplate(
                image: "xmark.rectangle.portrait",
                timestamp: timestamp,
                danger: false,
                line1: "Metadata removed",
                line2: ""//value.name + " version " +  value.version
            )
            case .networkSpecsAdded(let value): HistoryCardTemplate(
                image: "plus.viewfinder",
                timestamp: timestamp,
                danger: false,
                line1: "Network added",
                line2: ""//value
            )
            case .networkSpecsRemoved(let value): HistoryCardTemplate(
                image: "xmark.rectangle.portrait",
                timestamp: timestamp,
                danger: false,
                line1: "Network removed",
                line2: ""//value.title
            )
            case .networkVerifierSet(let value): HistoryCardTemplate(
                image: "checkmark.shield",
                timestamp: timestamp,
                danger: false,
                line1: "Network verifier set",
                line2: ""//value.genesisHash
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
                    line2: text.decode64()
                )
            case .seedNameWasShown(let text): HistoryCardTemplate(
                image: "eye.trianglebadge.exclamationmark.fill",
                timestamp: timestamp,
                danger: false,
                line1: "Seed was shown",
                line2: text.decode64()
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
                line2: ""//value.name + value.version
            )
            case .typesSigned(_): HistoryCardTemplate(
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
                line2: String(decoding: Data(base64Encoded: value.userComment) ?? Data(), as: UTF8.self)
            )
            case .transactionSigned(let value): HistoryCardTemplate(
                image: "signature",
                timestamp: timestamp,
                danger: false,
                line1: "Generated signature",
                line2: String(decoding: Data(base64Encoded: value.userComment) ?? Data(), as: UTF8.self)
            )
            case .typesAdded(_): HistoryCardTemplate(
                image: "plus.viewfinder",
                timestamp: timestamp,
                danger: false,
                line1: "New types info loaded",
                line2: ""
            )
            case .typesRemoved(_): HistoryCardTemplate(
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
            case .messageSignError(let _value): HistoryCardTemplate(
                image: "exclamationmark.triangle.fill",
                timestamp: timestamp,
                danger: true,
                line1: "Message signing error!",
                line2: ""
            )
            case .messageSigned(let value): HistoryCardTemplate(
                image: "signature",
                timestamp: timestamp,
                danger: false,
                line1: "Generated signature for message",
                line2: String(decoding: Data(base64Encoded: value.userComment) ?? Data(), as: UTF8.self)
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
