//
//  SettingsScreen.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 29.7.2021.
//

import SwiftUI

struct SettingsScreen: View {
    @EnvironmentObject var data: SignerDataModel
    @State var wipe = false
    var body: some View {
        ZStack {
            ScrollView {
                //Main buttons block
                VStack {
                    Button(action: {
                        data.settingsModal = .showSeedManager
                    }) {
                        Text("Manage seeds")
                    }.padding()
                    Button (action: {
                        data.settingsModal = .showNetworkManager
                    }) {
                        Text("Network settings")
                    }.padding()
                    Button(action: {
                        //TODO: add some alerts to make sure the operation was successful
                        wipe = true
                    }) {
                        HStack{
                            Image(systemName: "exclamationmark.triangle.fill").imageScale(.large)
                            Text("Wipe all data")
                            Image(systemName: "exclamationmark.triangle.fill").imageScale(.large)
                        }
                    }
                    .alert(isPresented: $wipe, content: {
                        Alert(
                            title: Text("Wipe ALL data?"),
                            message: Text("Factory reset the Signer app. This operation can not be reverted!"),
                            primaryButton: .cancel(),
                            secondaryButton: .destructive(
                                Text("Wipe"),
                                action: {
                                    data.wipe()
                                }
                            )
                        )
                    })
                    .padding()
                    Button(action: {
                        data.settingsModal = .showDocument(.about)
                    }) {
                        Text("About")
                    }
                    .padding()
                    Button(action: {
                        data.settingsModal = .showDocument(.toc)
                    }) {
                        Text("Terms and conditions")
                    }
                    .padding()
                    Button(action: {
                            data.settingsModal = .showDocument(.pp)}) {
                        Text("Privacy statement")
                    }
                    .padding()
                    Spacer()
                }
            }
            .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
            switch(data.settingsModal) {
            case .showSeedManager:
                SeedManager()
            case .showNetworkManager:
                NetworkManager()
            case .showDocument(let document):
                DocumentModal(document: document)
            case .none:
                EmptyView()
            }
        }
    }
}

/*
 struct SettingsScreen_Previews: PreviewProvider {
 static var previews: some View {
 NavigationView {
 SettingsScreen()
 }
 }
 }
 */
