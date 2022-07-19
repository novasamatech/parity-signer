//
//  VerifierScreen.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 20.12.2021.
//

import SwiftUI

struct VerifierScreen: View {
    @State var jailbreak = false
    let content: MVerifierDetails
    let doJailbreak: () -> Void
    var body: some View {
        VStack {
            HStack {
                Identicon(identicon: content.identicon, rowHeight: 42)
                VStack {
                    Text("General verifier certificate")
                    Text(content.publicKey)
                    Text("encryption: " + content.encryption)
                }
            }
            Button(
                action: {
                    jailbreak = true
                },
                label: {
                    SettingsCardTemplate(
                        text: "Remove general certificate",
                        danger: true
                    )
                })
                .alert(
                    isPresented: $jailbreak,
                    content: {
                        Alert(
                            title: Text("Wipe ALL data?"),
                            message: Text(
                                """
                                Remove all data and set general verifier blank so that it could be set later.
                                This operation can not be reverted.
                                Do not proceed unless you absolutely know what you are doing,
                                there is no need to use this procedure in most cases.
                                Misusing this feature may lead to loss of funds!
                                """
                            ),
                            primaryButton: .cancel(),
                            secondaryButton: .destructive(
                                Text("I understand"),
                                action: {
                                    doJailbreak()
                                }
                            )
                        )
                    }
                )
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
