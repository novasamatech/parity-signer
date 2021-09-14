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
                        ForEach(data.identities, id: \.public_key) {
                            identity in
                            IdentityCard(identity: identity)
                                .padding(.vertical, 2)
                        }
                    }
                }
                
                Spacer()
            }
            switch data.keyManagerModal {
            case .showKey:
                ExportIdentity()
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
