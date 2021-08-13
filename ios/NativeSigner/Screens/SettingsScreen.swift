//
//  SettingsScreen.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 29.7.2021.
//

import SwiftUI

struct SettingsScreen: View {
    @EnvironmentObject var data: SignerDataModel
    @Environment(\.presentationMode) var presentationMode: Binding<PresentationMode>
    @State var wipe = false
    @State var showHistory = false
    @State var showNetworkManager = false
    @State var showSeedManager = false
    var body: some View {
        ZStack {
            
            //Main buttons block
            VStack {
                Button(action: {
                    showHistory = true
                }) {
                    Text("Show log")
                }.padding()
                Button(action: {
                    showSeedManager = true
                }) {
                    Text("Manage seeds")
                }.padding()
                Button (action: {
                    showNetworkManager = true
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
                                presentationMode.wrappedValue.dismiss()
                            }
                        )
                    )
                })
                .padding()
                Button(action: {
                    data.document = .about
                }) {
                    Text("About")
                }
                .padding()
                Button(action: {
                    data.document = .toc
                }) {
                    Text("Terms and conditions")
                }
                .padding()
                Button(action: {data.document = .pp}) {
                    Text("Privacy statement")
                }
                .padding()
                Spacer()
            }.padding(.bottom, 100)
            
            //Modal with history
            if (showHistory) {
                HistoryView(showHistory: $showHistory)
            }
            
            //Modal with seed manager
            if (showSeedManager) {
                SeedManager(showSeedManager: $showSeedManager)
            }
            
            //Modal with network settings
            if (showNetworkManager) {
                NetworkManager(showNetworkManager: $showNetworkManager)
            }
            
            //Modal with information screens
            if (data.document != .none) {
                DocumentModal().padding(.bottom, 100)
            }
            
            //Footer
            VStack {
                Spacer()
                Footer(caller: "Settings")
            }
        }
        .navigationTitle("Settings").navigationBarTitleDisplayMode(.inline).toolbar {
            ToolbarItem(placement: .navigationBarTrailing) {
                NavbarShield()
            }
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
    }
}

struct SettingsScreen_Previews: PreviewProvider {
    static var previews: some View {
        NavigationView {
            SettingsScreen()
        }
    }
}
