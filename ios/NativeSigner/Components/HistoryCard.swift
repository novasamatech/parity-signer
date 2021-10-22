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
            case .databaseInitiated:
                HistoryCardTemplate(image: "1.square", timestamp: timestamp, color: "dangerColor", line1: "Database initiated", line2: "")
            case .deviceWasOnline:
                HistoryCardTemplate(image: "shield.slash", timestamp: timestamp, color: "dangerColor", line1: "Device was connected to network", line2: "")
            case .error(let text):
                HistoryCardTemplate(image: "exclamationmark.triangle.fill", timestamp: timestamp, color: "dangerColor", line1: "Error! " + text, line2: "")
            case .generalVerifierSet(let value):
                HistoryCardTemplate(image: "lock.shield.fill", timestamp: timestamp, color: "cryptoColor", line1: "General verifier set", line2: value.hex.truncateMiddle(length: 8) + "\n" + value.encryption)
            case .historyCleared:
                HistoryCardTemplate(image: "1.square", timestamp: timestamp, color: "dangerColor", line1: "History cleared", line2: "")
            case .identitiesWiped:
                HistoryCardTemplate(image: "key.filled", timestamp: timestamp, color: "cryptoColor", line1: "All keys were wiped", line2: "")
            case .identityAdded(let value):
                HistoryCardTemplate(image: "key", timestamp: timestamp, color: "cryptoColor", line1: "Key created", line2: value.seed_name + value.path)
            case .identityRemoved(let value):
                HistoryCardTemplate(image: "key", timestamp: timestamp, color: "cryptoColor", line1: "Key removed", line2: value.seed_name + value.path)
            case .metadataAdded(let value):
                HistoryCardTemplate(image: "plus.viewfinder", timestamp: timestamp, color: "cryptoColor", line1: "Metadata added", line2: value.specname + " version " +  value.spec_version)
            case .metadataRemoved(let value):
                HistoryCardTemplate(image: "minus.square", timestamp: timestamp, color: "cryptoColor", line1: "Metadata removed", line2: value.specname + " version " +  value.spec_version)
            case .networkAdded(let value):
                HistoryCardTemplate(image: "plus.viewfinder", timestamp: timestamp, color: "cryptoColor", line1: "Network added", line2: value.title)
            case .networkRemoved(let value):
                HistoryCardTemplate(image: "minus.square", timestamp: timestamp, color: "cryptoColor", line1: "Network removed", line2: value.title)
            case .networkVerifierSet(let value):
                HistoryCardTemplate(image: "lock.shield.fill", timestamp: timestamp, color: "cryptoColor", line1: "Network verifier set", line2: value.genesis_hash)
            case .resetDangerRecord:
                HistoryCardTemplate(image: "checkmark.shield", timestamp: timestamp, color: "dangerColor", line1: "Warnings acknowledged", line2: "")
            case .seedNameWasShown(let text):
                HistoryCardTemplate(image: "key", timestamp: timestamp, color: "cryptoColor", line1: "Seed was shown", line2: text)
            case .signedAddNetwork(_):
                HistoryCardTemplate(image: "pencil", timestamp: timestamp, color: "cryptoColor", line1: "Network specs signed", line2: "comment placeholder")
            case .signedLoadMetadata(_):
                HistoryCardTemplate(image: "pencil", timestamp: timestamp, color: "cryptoColor", line1: "Metadata signed", line2: "comment placeholder")
            case .signedTypes(_):
                HistoryCardTemplate(image: "pencil", timestamp: timestamp, color: "cryptoColor", line1: "Types signed", line2: "comment placeholder")
            case .systemEntry(let text):
                HistoryCardTemplate(image: "square", timestamp: timestamp, color: "cryptoColor", line1: "System record", line2: text)
            case .transactionSignError(let value):
                HistoryCardTemplate(image: "pencil", timestamp: timestamp, color: "cryptoColor", line1: "Generated signature", line2: String(decoding: Data(base64Encoded: value.user_comment) ?? Data(), as: UTF8.self))
            case .transactionSigned(let value):
                HistoryCardTemplate(image: "pencil", timestamp: timestamp, color: "cryptoColor", line1: "Generated signature", line2: String(decoding: Data(base64Encoded: value.user_comment) ?? Data(), as: UTF8.self))
            case .typesAdded(_):
                HistoryCardTemplate(image: "plus.viewfinder", timestamp: timestamp, color: "cryptoColor", line1: "New types info loaded", line2: "")
            case .typesRemoved(let value):
                HistoryCardTemplate(image: "minus.square", timestamp: timestamp, color: "dangerColor", line1: "Types info removed", line2: "")
            case .userEntry(let text):
                HistoryCardTemplate(image: "square", timestamp: timestamp, color: "cryptoColor", line1: "User record", line2: text)
            case .warning(let text):
                HistoryCardTemplate(image: "exclamationmark.triangle.fill", timestamp: timestamp, color: "dangerColor", line1: "Warning! " + text, line2: "")
            case .wrongPassword:
                HistoryCardTemplate(image: "exclamationmark.shield", timestamp: timestamp, color: "dangerColor", line1: "Wrong password entered", line2: "operation was declined")
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
