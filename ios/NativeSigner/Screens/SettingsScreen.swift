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
    @State var jailbreak = false
    let content: MSettings
    var body: some View {
        VStack (spacing: 2) {
            Button(action: {
                data.pushButton(action: .manageNetworks)
            }) {
                SettingsCardTemplate(text: "Networks")
            }
            Button(action: {
                data.pushButton(action: .backupSeed)
            }) {
                SettingsCardTemplate(text: "Backup keys")
            }
            Button(action: {data.pushButton(action: .viewGeneralVerifier)}) {
            VStack {
                HStack {
                    Text("Verifier certificate").font(FBase(style: .h1)).foregroundColor(Color("Text600"))
                    Spacer()
                }
                VStack {
                    if content.publicKey != nil {
                    AddressCard(address: Address(
                        base58: "encryption: " + (content.encryption ?? "unknown"), path: content.publicKey!.truncateMiddle(length: 8), hasPwd: false, identicon: content.identicon ?? [], seedName: "", multiselect: false
                    ))
                    } else {
                        if let errorMessage = content.error {
                            Text("Error!").foregroundColor(Color("SignalDanger")).font(FBase(style: .h4))
                            Text(errorMessage).foregroundColor(Color("SignalDanger")).font(FBase(style: .body2))
                        } else {
                            AddressCard(address: Address(
                                base58: "", path: "None", hasPwd: false, identicon: [], seedName: "", multiselect: false
                            ))
                        }
                    }
                }
            }
            .padding()
            }
            Button(action: {
                //TODO: add some alerts to make sure the operation was successful
                wipe = true
            }) {
                SettingsCardTemplate(
                    text: "Wipe all data",
                    danger: true
                )
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
            
            Button(action: {
                data.pushButton(action: .showDocuments)
            }) {
                SettingsCardTemplate(text: "About")
            }
            SettingsCardTemplate(
                text: "App version: " + (data.appVersion ?? "Unknown!"),
                withIcon: false,
                withBackground: false
            )
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
  
