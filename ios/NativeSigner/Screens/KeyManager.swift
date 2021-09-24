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
                HStack {
                    TextField("find keys", text: $data.searchKey)
                        .autocapitalization(.none)
                        .disableAutocorrection(true)
                        .font(.title)
                        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("textFieldColor")/*@END_MENU_TOKEN@*/)
                        .foregroundColor(/*@START_MENU_TOKEN@*/Color("textEntryColor")/*@END_MENU_TOKEN@*/)
                        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("textFieldColor")/*@END_MENU_TOKEN@*/)
                        .border(/*@START_MENU_TOKEN@*/Color("borderSignalColor")/*@END_MENU_TOKEN@*/, width: /*@START_MENU_TOKEN@*/1/*@END_MENU_TOKEN@*/)
                    if (data.searchKey != "") {
                        Button(action:{data.searchKey = ""}) {
                            Image(systemName: "clear").imageScale(.large)
                        }
                    } else {
                        Image(systemName: "doc.text.magnifyingglass").imageScale(.large).foregroundColor(Color("AccentColor"))
                    }
                }.padding(.horizontal)
                ScrollView {
                    LazyVStack {
                        ForEach(data.identities, id: \.public_key) {
                            identity in
                            if (identity.name.contains(data.searchKey) || identity.path.contains(data.searchKey) || data.searchKey == "" ) {
                                IdentityCard(identity: identity)
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
