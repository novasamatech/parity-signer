//
//  NetworkManager.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 12.8.2021.
//

import SwiftUI

struct NetworkManager: View {
    @EnvironmentObject var data: SignerDataModel
    @Binding var showNetworkManager: Bool
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 50).foregroundColor(/*@START_MENU_TOKEN@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
            VStack {
                NetworkList()
                VStack(alignment: .leading) {
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
                Button(action: {
                    showNetworkManager = false
                }) {
                    Text("Done")
                }
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
        .padding(.bottom, 100)
    }
}

/*
struct NetworkManager_Previews: PreviewProvider {
    static var previews: some View {
        NetworkManager()
    }
}
*/
