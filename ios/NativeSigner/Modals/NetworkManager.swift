//
//  NetworkManager.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 12.8.2021.
//

import SwiftUI

struct NetworkManager: View {
    let content: MNetworkMenu
    let pushButton: (Action, String, String) -> Void
    var body: some View {
        VStack {
            Rectangle().frame(height: UIScreen.main.bounds.height/3).opacity(0.0001).gesture(TapGesture().onEnded {_ in
                pushButton(.goBack, "", "")
            })
            ZStack {
                RoundedRectangle(cornerRadius: 20.0).foregroundColor(Color("Bg000"))
                VStack {
                    Spacer()
                    Rectangle().foregroundColor(Color("Bg000")).frame(height: 25)
                }
                VStack {
                    HeaderBar(line1: "NETWORK", line2: "Select network").padding(10)
                    ScrollView {
                        LazyVStack {
                            ForEach(content.networks.sorted(by: {$0.order < $1.order}), id: \.order) {network in
                                ZStack {
                                    Button(
                                        action: {
                                            pushButton(.changeNetwork, network.key, "")
                                        },
                                        label: {
                                            NetworkCard(title: network.title, logo: network.logo, fancy: true)
                                        })
                                    HStack {
                                        Spacer()
                                        if network.selected {
                                            Image(systemName: "checkmark")
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
