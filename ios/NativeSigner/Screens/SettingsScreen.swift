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
    let content: MVerifierDetails
    var body: some View {
        VStack (spacing: 2) {
            Button(action: {
                data.pushButton(buttonID: .ManageNetworks)
            }) {
                SettingsCardTemplate(text: "Networks")
            }
            Button(action: {
                data.pushButton(buttonID: .BackupSeed)
            }) {
                SettingsCardTemplate(text: "Backup keys")
            }
            Button(action: {data.pushButton(buttonID: .ViewGeneralVerifier)}) {
            VStack {
                HStack {
                    Text("Verifier certificate").font(FBase(style: .h1)).foregroundColor(Color("Text600"))
                    Spacer()
                }
                HStack {
                    AddressCard(address: Address(
                        base58: "encryption: " + content.encryption, path: content.hex.truncateMiddle(length: 8), has_pwd: false, identicon: content.identicon, seed_name: "", multiselect: false
                    ))
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
                data.pushButton(buttonID: .ShowDocuments)
            }) {
                SettingsCardTemplate(text: "About")
            }
            /*
            SettingsCardTemplate(
                text: "App version: " + (data.appVersion ?? "Unknown!"),
                withIcon: false,
                withBackground: false
            )
             */
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
  
