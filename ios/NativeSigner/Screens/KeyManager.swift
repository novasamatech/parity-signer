//
//  KeyList.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import SwiftUI

struct KeyManager: View {
    @EnvironmentObject var data: SignerDataModel
    @GestureState private var dragOffset = CGSize.zero
    var content: MKeys
    var body: some View {
        ZStack {
            VStack {
                Button(action: {
                    data.pushButton(buttonID: .SelectKey, details: content.root.address_key)
                }){
                    SeedKeyCard(seedCard: content.root)
                }.padding(2)
                Button(action: {data.pushButton(buttonID: .NetworkSelector)}) {
                    NetworkCard(title: content.network.title, logo: content.network.logo)
                }
                HStack {
                        Text("DERIVED KEYS").foregroundColor(Color("Text600"))
                        Spacer()
                        Button(action: {
                            data.pushButton(buttonID: .NewKey)
                        }) {
                            Image(systemName: "plus.circle").imageScale(.large).foregroundColor(Color("Action400"))
                        }
                    }.padding(.horizontal, 8)
                ScrollView {
                    LazyVStack {
                        ForEach(content.set, id: \.address_key) {
                            address in
                            Button(action: {
                                data.pushButton(buttonID: .SelectKey, details: address.address_key)
                            }){
                                AddressCard(address: address.intoAddress())
                            }.padding(2)
                        }
                    }
                }
                Spacer()
            }
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
    }
}

/*
 struct KeyManager_Previews: PreviewProvider {
 static var previews: some View {
 NavigationView {
 KeyManager()
 }
 }
 }
 */
