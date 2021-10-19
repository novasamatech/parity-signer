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
            case .generalVerifierAdded(let value):
                HistoryCardTemplate(image: "lock.shield.fill", timestamp: timestamp, color: "cryptoColor", line1: "General verifier set", line2: value.verifier.hex.prefix(4) + " " + value.verifier.encryption)
            case .generalVerifierRemoved(_):
                HistoryCardTemplate(image: "lock.slash", timestamp: timestamp, color: "dangerColor", line1: "General verifier unset", line2: "Signer wiped")
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
            case .metadataVerifierAdded(let value):
                HistoryCardTemplate(image: "lock.shield.fill", timestamp: timestamp, color: "cryptoColor", line1: "Network verifier set", line2: value.specname)
            case .metadataVerifierRemoved(_):
                HistoryCardTemplate(image: "lock.shield.fill", timestamp: timestamp, color: "cryptoColor", line1: "Network verifier was cleared", line2: "this is error, report a bug") //TODO: this should not be possible!
            case .networkAdded(let value):
                HistoryCardTemplate(image: "plus.viewfinder", timestamp: timestamp, color: "cryptoColor", line1: "Network added", line2: value.specname)
            case .networkRemoved(let value):
                HistoryCardTemplate(image: "minus.square", timestamp: timestamp, color: "cryptoColor", line1: "Network removed", line2: value.title)
            case .seedNameWasAccessed(let text):
                HistoryCardTemplate(image: "key", timestamp: timestamp, color: "cryptoColor", line1: "Seed was accessed", line2: text)
            case .seedNameWasShown(let text):
                HistoryCardTemplate(image: "key", timestamp: timestamp, color: "cryptoColor", line1: "Seed was shown", line2: text)
            case .seedsWereAccessed:
                HistoryCardTemplate(image: "key", timestamp: timestamp, color: "cryptoColor", line1: "Seeds were accessed", line2: "")
            case .seedsWereShown:
                HistoryCardTemplate(image: "key", timestamp: timestamp, color: "cryptoColor", line1: "Seeds were shown", line2: "")
            case .signedAddNetwork(_):
                HistoryCardTemplate(image: "pencil", timestamp: timestamp, color: "cryptoColor", line1: "Network specs signed", line2: "comment placeholder")
            case .signedLoadMetadata(_):
                HistoryCardTemplate(image: "pencil", timestamp: timestamp, color: "cryptoColor", line1: "Metadata signed", line2: "comment placeholder")
            case .signedTypes(_):
                HistoryCardTemplate(image: "pencil", timestamp: timestamp, color: "cryptoColor", line1: "Types signed", line2: "comment placeholder")
            case .systemEntry(let text):
                HistoryCardTemplate(image: "square", timestamp: timestamp, color: "cryptoColor", line1: "System record", line2: text)
            case .transactionSigned(let value):
                HistoryCardTemplate(image: "pencil", timestamp: timestamp, color: "cryptoColor", line1: "Generated signature", line2: String(decoding: Data(base64Encoded: value.user_comment) ?? Data(), as: UTF8.self))
            case .typesInfoUpdated(_):
                HistoryCardTemplate(image: "plus.viewfinder", timestamp: timestamp, color: "cryptoColor", line1: "New types info loaded", line2: "")
            case .userEntry(let text):
                HistoryCardTemplate(image: "square", timestamp: timestamp, color: "cryptoColor", line1: "User record", line2: text)
            case .warning(let text):
                HistoryCardTemplate(image: "exclamationmark.triangle.fill", timestamp: timestamp, color: "dangerColor", line1: "Warning! " + text, line2: "")
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
