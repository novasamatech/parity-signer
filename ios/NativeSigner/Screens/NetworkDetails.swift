//
//  NetworkDetails.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 13.10.2021.
//

import SwiftUI

struct NetworkDetails: View {
    @EnvironmentObject var data: SignerDataModel
    var content: MNetworkDetails
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
                        Text(content.genesisHash)
                    }
                    HStack {
                        Text("Verifier certificate: ")
                        switch content.currentVerifier.ttype {
                        case "general":
                            Text("General")
                        case "network":
                            VStack {
                                Text("custom")
                                Text(String(describing: content.currentVerifier.details))
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
                        ForEach(content.meta, id: \.metaHash) {
                            metaEntry in
                            Button(
                                action: {
                                    data.pushButton(action: .manageMetadata, details: metaEntry.specsVersion)
                                }
                            ){
                            MetadataCard(meta: metaEntry)
                            }
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
