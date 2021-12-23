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
                    Text("Verifier certificate").font(FBase(style: .h1))
                    Spacer()
                }
                HStack {
                    Image(uiImage: UIImage(data: Data(fromHexEncodedString: content.identicon) ?? Data()) ?? UIImage())
                    .resizable(resizingMode: .stretch)
                    .frame(width: 28, height: 28)
                    VStack{
                        Text(content.hex.truncateMiddle(length: 8))
                        Text("encryption: " + content.encryption)
                    }.foregroundColor(Color("Crypto400"))
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
                //TODO: add some alerts to make sure the operation was successful
                jailbreak = true
            }) {
                SettingsCardTemplate(
                    text: "Remove general certificate",
                    danger: true
                )
            }
            .alert(isPresented: $jailbreak, content: {
                Alert(
                    title: Text("Wipe ALL data?"),
                    message: Text("Remove all data and set general verifier blank so that it could be set later. This operation can not be reverted. Do not proceed unless you absolutely know what you are doing, there is no need to use this procedure in most cases. Misusing this feature may lead to loss of funds!"),
                    primaryButton: .cancel(),
                    secondaryButton: .destructive(
                        Text("I understand"),
                        action: {
                            data.jailbreak()
                        }
                    )
                )
            })
            Button(action: {
                //TODO
            }) {
                SettingsCardTemplate(text: "About")
            }
            SettingsCardTemplate(
                text: "App version: " + (data.appVersion ?? "Unknown!"),
                withIcon: false,
                withBackground: false
            )
            /*
             HStack {
             Image(uiImage: UIImage(data: Data(fromHexEncodedString: String(cString: identicon(nil, "", 32))) ?? Data()) ?? UIImage())
             .resizable(resizingMode: .stretch)
             .frame(width: 42, height: 42)
             VStack {
             Text("General verifier certificate").foregroundColor(Color("textMainColor"))
             //Text(data.generalVerifier?.hex ?? "unknown").foregroundColor(Color("cryptoColor"))
             //Text("encryption: " + (data.generalVerifier?.encryption ?? "unknown")).foregroundColor(Color("textFadedColor"))
             }
             }.padding().background(Color("backgroundCard"))
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
  
