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
                HStack {
                    NetworkList()
                    SeedSelector()
                }
                ScrollView {
                    LazyVStack {
                        ForEach(data.addresses, id: \.public_key) {
                            address in
                            if (address.name.contains(data.searchKey) || address.path.contains(data.searchKey) || data.searchKey == "" ) {
                                AddressCard(identity: address)
                                    .padding(.vertical, 2)
                                
                            }
                        }
                        if data.selectedSeed != "" {
                            Button(action:{
                                data.proposeDerive()
                                data.keyManagerModal = .newKey
                            }) {
                                Text("Add key")
                                    .font(.largeTitle)
                                    .foregroundColor(Color("AccentColor"))
                                    .multilineTextAlignment(.center)
                            }
                        }
                    }
                }
                
                Spacer()
                SearchKeys()
            }
            switch data.keyManagerModal {
            case .showKey:
                ExportAddress()
            case .newKey:
                NewIdentityScreen()
            case .newSeed:
                NewSeedScreen()
            case .seedBackup:
                SeedBackup()
            case .keyDeleteConfirm:
                //not used yet - primitive alert dialog works well enough
                EmptyView()
            case .none:
                EmptyView()
                
            }
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
        .onAppear {
            data.totalRefresh()
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
