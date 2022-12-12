//
//  VerifierScreen.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 20.12.2021.
//

import SwiftUI

struct VerifierScreen: View {
    @State private var jailbreak = false
    let content: MVerifierDetails
    let doJailbreak: () -> Void
    var body: some View {
        VStack {
            HStack {
                Identicon(identicon: content.identicon, rowHeight: 42)
                VStack {
                    Localizable.generalVerifierCertificate.text
                    Text(content.publicKey)
                    Text(Localizable.encryption(content.encryption))
                }
            }
            Button(
                action: {
                    jailbreak = true
                },
                label: {
                    SettingsCardTemplate(
                        text: Localizable.removeGeneralCertificate.key,
                        danger: true
                    )
                }
            )
            Spacer()
        }
        .alert(
            isPresented: $jailbreak,
            content: {
                Alert(
                    title: Localizable.wipeALLData.text,
                    message: Localizable.RemoveAllData.message.text,
                    primaryButton: .cancel(),
                    secondaryButton: .destructive(
                        Localizable.iUnderstand.text,
                        action: {
                            doJailbreak()
                        }
                    )
                )
            }
        )
    }
}

// struct VerifierScreen_Previews: PreviewProvider {
// static var previews: some View {
// VerifierScreen()
// }
// }
