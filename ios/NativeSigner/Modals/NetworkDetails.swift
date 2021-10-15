//
//  NetworkDetails.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 13.10.2021.
//

import SwiftUI

struct NetworkDetails: View {
    @EnvironmentObject var data: SignerDataModel
    var body: some View {
        ZStack {
            ModalBackdrop()
            VStack {
                VStack(alignment: .leading) {
                    HStack {
                        Text("Network name:")
                            .foregroundColor(Color("AccentColor"))
                        Text(data.networkSettings?.title ?? "unknown")
                            .foregroundColor(Color("textMainColor"))
                    }
                    HStack {
                        Text("base58 prefix:")
                            .foregroundColor(Color("AccentColor"))
                        Text(data.networkSettings?.base58prefix ?? "unknown")
                            .foregroundColor(Color("textMainColor"))
                    }
                    HStack {
                        Text("decimals:")
                            .foregroundColor(Color("AccentColor"))
                        Text(data.networkSettings?.decimals ?? "unknown")
                            .foregroundColor(Color("textMainColor"))
                    }
                    HStack {
                        Text("unit:")
                            .foregroundColor(Color("AccentColor"))
                        Text(data.networkSettings?.unit ?? "unknown")
                            .foregroundColor(Color("textMainColor"))
                    }
                    HStack {
                        Text("genesis hash:")
                            .foregroundColor(Color("AccentColor"))
                        Text(data.networkSettings?.genesis_hash ?? "unknown")
                            .foregroundColor(Color("textMainColor"))
                    }
                }
                Text("Metadata available:")
                    .font(.title)
                    .foregroundColor(Color("AccentColor"))
                ScrollView {
                    LazyVStack {
                        ForEach(data.networkSettings?.meta ?? [], id: \.meta_hash) {
                            metaEntry in
                            MetadataCard(meta: metaEntry)
                        }
                    }
                }
                Spacer()
            }.padding()
        }
        .onAppear {
            data.getNetworkSettings()
        }
        .onReceive(data.$selectedNetwork, perform: { _ in
            data.getNetworkSettings()
        })
        .onDisappear {
            data.networkSettings = nil
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
