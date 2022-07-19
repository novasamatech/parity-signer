//
//  NetworkDetails.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 13.10.2021.
//

import SwiftUI

struct NetworkDetails: View {
    let content: MNetworkDetails
    let pushButton: (Action, String, String) -> Void
    var body: some View {
        ZStack {
            VStack {
                VStack(alignment: .leading) {
                    NetworkCard(title: content.title, logo: content.logo)
                    HStack {
                        Text("Network name:")
                        Text(content.name)
                    }
                    HStack {
                        Text("base58 prefix:")
                        Text(String(content.base58prefix))
                    }
                    HStack {
                        Text("decimals:")
                        Text(String(content.decimals))
                    }
                    HStack {
                        Text("unit:")
                        Text(content.unit)
                    }
                    HStack {
                        Text("genesis hash:")
                        Text(content.genesisHash.map {String(format: "%02X", $0)}.joined())
                            .fixedSize(horizontal: false, vertical: true)
                    }
                    HStack {
                        Text("Verifier certificate: ").fixedSize(horizontal: false, vertical: true)
                        switch content.currentVerifier.ttype {
                        case "general":
                            Text("general")
                        case "custom":
                            Identicon(identicon: content.currentVerifier.details.identicon)
                            VStack {
                                Text("custom")
                                Text(content.currentVerifier.details.publicKey)
                                    .fixedSize(horizontal: false, vertical: true)
                                Text("encryption: " + content.currentVerifier.details.encryption)
                            }
                        case "none":
                            Text("none")
                        default:
                            Text("unknown")
                        }
                    }
                }
                Text("Metadata available:")
                ScrollView {
                    LazyVStack {
                        ForEach(content.meta, id: \.metaHash) { metaEntry in
                            Button(
                                action: {
                                    pushButton(.manageMetadata, metaEntry.specsVersion, "")
                                },
                                label: {
                                    MetadataCard(meta: metaEntry)
                                }
                            )
                        }
                    }
                }
                Spacer()
            }.padding()
        }
    }
}

/*
 struct NetworkDetails_Previews: PreviewProvider {
 static var previews: some View {
 NetworkDetails()
 }
 }
 */
