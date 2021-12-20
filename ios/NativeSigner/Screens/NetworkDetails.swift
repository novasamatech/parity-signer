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
                        Text(content.base58prefix)
                    }
                    HStack {
                        Text("decimals:")
                        Text(content.decimals)
                    }
                    HStack {
                        Text("unit:")
                        Text(content.unit)
                    }
                    HStack {
                        Text("genesis hash:")
                        Text(content.genesis_hash)
                    }
                    HStack {
                        Text("Verifier certificate: ")
                        switch content.current_verifier.type {
                        case "general":
                            Text("General")
                        case "network":
                            VStack {
                                Text("custom")
                                Text(String(describing: content.current_verifier.details))
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
                        ForEach(content.meta, id: \.meta_hash) {
                            metaEntry in
                            Button(
                                action: {
                                    data.pushButton(buttonID: .ManageMetadata, details: metaEntry.spec_version)
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
