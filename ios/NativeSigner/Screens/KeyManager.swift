//
//  KeyList.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import SwiftUI

struct KeyManager: View {
    @EnvironmentObject var data: SignerDataModel
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
                            Image(systemName: "plus.square.on.square").imageScale(.large)
                        }
                    }
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
                NetworkManager().frame(height: UIScreen.main.bounds.height - 90.0).offset(y: UIScreen.main.bounds.height - 300.0)
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
