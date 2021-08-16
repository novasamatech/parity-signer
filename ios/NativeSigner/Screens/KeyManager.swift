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
                NetworkList()
                SeedSelector()
                ScrollView {
                    LazyVStack {
                        ForEach(data.identities, id: \.public_key) {
                            identity in
                            IdentityCard(identity: identity)
                                .padding(.vertical, 2)
                        }
                    }
                }
                Spacer()
            }.padding(.bottom, 100)
            
            //Modal to export public key
            if data.exportIdentity {
                ExportIdentity()
            }
            
            //Modal to create new key
            if data.newIdentity {
                NewIdentityScreen()
            }
            
            //Modal to create new seed
            if data.newSeed {
                NewSeedScreen()
            }
            VStack {
                Spacer()
                Footer(caller: "KeyManager")
            }
    }
        .navigationTitle("Manage identities")
        .navigationBarTitleDisplayMode(.inline)
        .toolbar {
            ToolbarItem(placement: .navigationBarTrailing) {
                NavbarShield()
            }
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
        .onAppear {
            data.totalRefresh()
        }
    }
}

struct KeyManager_Previews: PreviewProvider {
    static var previews: some View {
        NavigationView {
            KeyManager()
        }
    }
}
