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
                SeedSelector()
                NetworkSelector()
                if data.selectedSeed != "" {
                    HStack {
                        Text("DERIVED KEYS").foregroundColor(Color("textFadedColor"))
                        Spacer()
                        Button(action: {
                            data.proposeDerive()
                            data.keyManagerModal = .newKey
                        }) {
                            Image(systemName: "plus.circle").imageScale(.large)
                        }
                    }.padding(.horizontal, 8)
                }
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
                
                Spacer()
            }
            switch data.keyManagerModal {
            case .showKey:
                ExportAddress()
            case .newKey:
                NewAddressScreen()
            case .newSeed:
                NewSeedScreen()
            case .seedBackup:
                SeedBackup()
            case .keyDeleteConfirm:
                //not used yet - primitive alert dialog works well enough
                EmptyView()
            case .seedSelector:
                SeedManager()
            case .networkManager:
                VStack {
                    Spacer()
                    NetworkManager().frame(height: UIScreen.main.bounds.height*0.4)
                }.gesture(DragGesture().updating($dragOffset, body: { (value, state, transaction) in
                    if value.translation.height > 100 {
                        data.goBack()
                    }
                }))
            case .networkDetails:
                NetworkDetails()
            case .none:
                EmptyView()
            }
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
        .onAppear {
            //data.totalRefresh()
        }
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
