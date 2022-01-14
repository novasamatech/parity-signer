//
//  VerifierScreen.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 20.12.2021.
//

import SwiftUI

struct VerifierScreen: View {
    @EnvironmentObject var data: SignerDataModel
    @State var jailbreak = false
    let content: MVerifierDetails
    var body: some View {
        VStack {
            HStack {
                Image(uiImage: UIImage(data: Data(fromHexEncodedString: content.identicon) ?? Data()) ?? UIImage())
                    .resizable(resizingMode: .stretch)
                    .frame(width: 42, height: 42)
                VStack{
                    Text("General verifier certificate")
                    Text(content.hex)
                    Text("encryption: " + content.encryption)
                }
            }
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
        }
    }
}
    
    
    /*
     struct VerifierScreen_Previews: PreviewProvider {
     static var previews: some View {
     VerifierScreen()
     }
     }
     */
