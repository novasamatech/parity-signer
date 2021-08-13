//
//  HistoryCollapsed.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 13.8.2021.
//

import SwiftUI

struct HistoryCollapsed: View {
    var history: History
    var body: some View {
        VStack {
            HStack {
                Text(history.timestamp)
                Spacer()
            }
            VStack(alignment: .trailing) {
                ForEach(history.events, id: \.self) {event in
                    HStack {
                        Spacer()
                        switch event {
                        case .databaseInitiated:
                            Text("Database initiated")
                                .foregroundColor(Color.red)
                        case .deviceWasOnline:
                            Text("Device was online!")
                                .foregroundColor(Color.red)
                        case .error(let text):
                            Text("Error! " + text)
                                .foregroundColor(Color.red)
                        case .generalVerifierAdded(_):
                            Text("New general verifier key")
                        case .generalVerifierRemoved(_):
                            Text("Removed general verifier key")
                        case .historyCleared:
                            Text("History cleared")
                                .foregroundColor(Color.red)
                        case .identitiesWiped:
                            Text("Identities wipe")
                        case .identityAdded(_):
                            Text("New identity created")
                        case .identityRemoved(_):
                            Text("Removed identity")
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
        }
        .foregroundColor(/*@START_MENU_TOKEN@*/Color("textMainColor")/*@END_MENU_TOKEN@*/)
        .padding()
    }
}

/*
struct HistoryCollapsed_Previews: PreviewProvider {
    static var previews: some View {
        HistoryCollapsed()
    }
}
*/
