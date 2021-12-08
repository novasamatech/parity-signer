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
    var body: some View {
        ZStack {
            VStack {
                //SeedSelector()
                //NetworkSelector()
                if data.selectedSeed != "" {
                    HStack {
                        Text("DERIVED KEYS").foregroundColor(Color("textFadedColor"))
                        Spacer()
                        Button(action: {
                            //TODO
                        }) {
                            Image(systemName: "plus.circle").imageScale(.large)
                        }
                    }.padding(.horizontal, 8)
                }
                /*
                ScrollView {
                    LazyVStack {
                        ForEach(data.addresses, id: \.public_key) {
                            address in
                            if ((address.name.contains(data.searchKey) || address.path.contains(data.searchKey) || data.searchKey == "" ) && (!address.isRoot() || data.selectedSeed == "")) {
                                AddressCard(address: address)
                                //.padding(.vertical, 2)
                            }
                        }
                    }
                }
                */
                Text("Keys")
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
