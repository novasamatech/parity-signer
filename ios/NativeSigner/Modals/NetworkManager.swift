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
        VStack {
            ZStack {
                RoundedRectangle(cornerRadius: 20.0).foregroundColor(Color("backgroundNetworkModal"))
                VStack {
                    Spacer()
                    Rectangle().foregroundColor(Color("backgroundNetworkModal")).frame(height: 25)
                }
                VStack {
                    HeaderBar(line1: "NETWORK", line2: "Select network").padding(.top, 10)
                    ScrollView {
                        LazyVStack {
                            ForEach(data.networks, id: \.self) {network in
                                ZStack (alignment: .bottom) {
                                    RoundedRectangle(cornerRadius: 20).foregroundColor(Color(data.selectedNetwork == network ? "backgroundActive" : "backgroundNetworkModal"))
                                    HStack {
                                        Button(action: {
                                            data.selectNetwork(network: network)
                                        }) {
                                            NetworkCard(network: network)
                                        }
                                        Spacer()
                                        if network == data.selectedNetwork {
                                            Button(action: {
                                                data.keyManagerModal = .networkDetails
                                            }) {
                                                Image(systemName: "eye").imageScale(.large)
                                            }
                                            Button(action: {}) {
                                                Image(systemName: "trash").imageScale(.large)
                                            }
                                        }
                                    }.padding(.horizontal, 8)
                                }.padding(.horizontal, 8)
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
