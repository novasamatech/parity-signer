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
    var body: some View {
        ZStack {
            //ScrollView {
            //Main buttons block
            VStack {
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
                    //TODO: add some alerts to make sure the operation was successful
                    jailbreak = true
                }) {
                    HStack{
                        Image(systemName: "exclamationmark.triangle.fill").imageScale(.large)
                        Text("Remove general certificate")
                        Image(systemName: "exclamationmark.triangle.fill").imageScale(.large)
                    }.foregroundColor(Color("SignalDanger"))
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
                .padding()
                Button(action: {
                    //TODO
                }) {
                    Text("Documentation")
                }
                .padding()
                HStack {
                    Text("App version:")
                    Text(data.appVersion ?? "Unknown!")
                }
                Spacer()
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
            .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
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
