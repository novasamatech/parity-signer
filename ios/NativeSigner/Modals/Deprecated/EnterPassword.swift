//
//  EnterPassword.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 15.12.2021.
//

import SwiftUI

struct EnterPassword: View {
    var content: MEnterPassword
    let navigationRequest: NavigationRequest
    @State private var password: String = ""
    @FocusState private var focused: Bool
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 20.0).foregroundColor(Asset.bg000.swiftUIColor)
            VStack {
                HeaderBar(line1: Localizable.secretPath.key, line2: Localizable.Path.password.key)
                AddressCard(address: content.authorInfo)
                if content.counter > 0 {
                    Text(Localizable.remainingAttempts(String(content.counter)))
                }
                ZStack {
                    RoundedRectangle(cornerRadius: 8)
                        .stroke(Asset.crypto400.swiftUIColor)
                        .frame(height: 39)
                    HStack {
                        Localizable.Path.delimeter.text
                            .foregroundColor(Asset.crypto400.swiftUIColor)
                        TextField(Localizable.secretPath.string, text: $password, prompt: Text(""))
                            .foregroundColor(Asset.crypto400.swiftUIColor)
                            .font(Fontstyle.body2.crypto)
                            .disableAutocorrection(true)
                            .autocapitalization(.none)
                            .keyboardType(.asciiCapable)
                            .submitLabel(.done)
                            .focused($focused)
                            .padding(8)
                            .onAppear {
                                focused = true
                            }
                    }
                }
                BigButton(
                    text: Localizable.next.key,
                    isCrypto: true,
                    action: {
                        navigationRequest(.init(action: .goForward, details: password))
                    },
                    isDisabled: password.isEmpty
                )
            }
        }
    }
}

// struct EnterPassword_Previews: PreviewProvider {
// static var previews: some View {
// EnterPassword()
// }
// }
