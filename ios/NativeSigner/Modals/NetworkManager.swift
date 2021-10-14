//
//  NetworkManager.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 12.8.2021.
//

import SwiftUI

struct NetworkManager: View {
    @EnvironmentObject var data: SignerDataModel
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 50).foregroundColor(Color("backgroundCard"))
            VStack {
                Text("Select network").font(.largeTitle)
                ScrollView {
                    LazyVStack {
                        ForEach(data.networks, id: \.self) {network in
                            HStack {
                                Button(action: {
                                    data.selectNetwork(network: network)
                                    data.goBack()
                                }) {
                                    NetworkCard(network: network)
                                }
                                Spacer()
                                Button(action: {}) {
                                    Image(systemName: "eye")
                                }
                                Button(action: {}) {
                                    Image(systemName: "trash")
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/*
struct NetworkManager_Previews: PreviewProvider {
    static var previews: some View {
        NetworkManager()
    }
}
*/
