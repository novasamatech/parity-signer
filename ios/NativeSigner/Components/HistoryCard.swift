//
//  HistoryCard.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 13.8.2021.
//

import SwiftUI

struct HistoryCard: View {
    var event: Event
    var body: some View {
        VStack {
            switch event {
            case .databaseInitiated:
                Text("Database initiated")
                    .foregroundColor(Color.red)
                Text("The Signer was factory-reset here.")
            case .deviceWasOnline:
                Text("Device was online!")
                    .foregroundColor(Color.red)
                Text("You might want to consider all keys compromised at this point; please follow your security protocol")
                //TODO: add nervous icon and this button function
                Button(action: {}) {
                    Text("Acknowledge and dismiss")
                        .font(.largeTitle)
                        .foregroundColor(Color("AccentColor"))
                }
            case .error(let text):
                Text("Error! " + text)
                    .foregroundColor(Color.red)
            case .generalVerifierAdded(let value):
                Text("New general verifier key")
                HStack {
                    Text("Encryption algorithm: ")
                    Text(value.encryption)
                }
                HStack {
                    Text("Public key: ")
                    Text(value.hex)
                }
                Text("This is signature of trusted party that verifies general updates for this Signer from now on. No other sources of updates will be accepted unless you reset this key.")
            case .generalVerifierRemoved(let value):
                Text("Removed general verifier key")
                HStack {
                    Text("Encryption algorithm: ")
                    Text(value.encryption)
                }
                HStack {
                    Text("Public key: ")
                    Text(value.hex)
                }
                Text("You have removed the key of trusted party that could verify updates for this Signer. This is very rare operation you should not have performed without knowing ecavtly its purpose and consequences. Please proceed with great care.")
            case .historyCleared:
                Text("History cleared")
                    .foregroundColor(Color.red)
                Text("You've cleared the log here; there should be no records below this one.")
            case .identitiesWiped:
                Text("Identities wipe")
                Text("You have deleted all your identities here")
            case .identityAdded(let value):
                Text("New identity created")
                HStack {
                    Text("Seed name: ")
                    Text(value.seed_name)
                }
                HStack {
                    Text("Public key: ")
                    Text(value.public_key)
                }
                HStack {
                    Text("Derivation path: ")
                    Text(value.path)
                }
                HStack {
                    Text("Network key: ")
                    Text(value.network_genesis_hash)
                }
                //TODO: fill below
            case .identityRemoved(let value):
                Text("Removed identity")
                HStack {
                    Text("Seed name: ")
                    Text(value.seed_name)
                }
                HStack {
                    Text("Public key: ")
                    Text(value.public_key)
                }
                HStack {
                    Text("Derivation path: ")
                    Text(value.path)
                }
                HStack {
                    Text("Network key: ")
                    Text(value.network_genesis_hash)
                }
            case .metadataAdded(_):
                Text("New metadata loaded")
            case .metadataRemoved(_):
                Text("Remove metadata")
            case .metadataVerifierAdded(_):
                Text("New network verifier accepted")
            case .metadataVerifierRemoved(_):
                Text("Removed network verifier")
            case .networkAdded(_):
                Text("New network loaded")
            case .networkRemoved(_):
                Text("Removed network")
            case .seedNameWasAccessed(let text):
                Text("Seed was accessed: " + text)
            case .seedNameWasShown(let text):
                Text("Seed was shown: " + text)
            case .seedsWereAccessed:
                Text("Seeds were accessed")
            case .seedsWereShown:
                Text("Seeds were shown")
            case .signedAddNetwork(_):
                Text("Network specs signed")
            case .signedLoadMetadata(_):
                Text("Metadata signed")
            case .signedTypes(_):
                Text("Type specs signed")
            case .systemEntry(let text):
                Text("System: " + text)
            case .transactionSigned(_):
                Text("Transaction signed")
            case .typesInfoUpdated(_):
                Text("New types information loaded")
            case .userEntry(let text):
                Text("Note: " + text)
            case .warning(let text):
                Text("Warning: " + text)
                    .foregroundColor(Color.red)
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
