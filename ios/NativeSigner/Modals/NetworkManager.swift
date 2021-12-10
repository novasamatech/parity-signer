//
//  NetworkManager.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 12.8.2021.
//

import SwiftUI

struct NetworkManager: View {
    @EnvironmentObject var data: SignerDataModel
    var content: MNetworkMenu
    var body: some View {
        VStack {
                Spacer().frame(height: UIScreen.main.bounds.height/2)
            ZStack {
                RoundedRectangle(cornerRadius: 20.0).foregroundColor(Color("Bg000"))
                VStack {
                    Spacer()
                    Rectangle().foregroundColor(Color("Bg000")).frame(height: 25)
                }
                VStack {
                    HeaderBar(line1: "NETWORK", line2: "Select network").padding(.top, 10)
                    /*
                    ScrollView {
                        LazyVStack {
                            ForEach(content.networks, id: \.self) {network in
                                ZStack (alignment: .bottom) {
                                    HStack {
                                        Button(action: {
                                           
                                        }) {
                                            //NetworkCard(network.toCard())
                                        }
                                        Spacer()
                                        if true {
                                            Image(systemName: "checkmark")
                                        }
                                    }.padding(.horizontal, 8)
                                }.padding(.horizontal, 8)
                            }
                        }
                    }*/
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
