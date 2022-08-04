//
//  EnterPassword.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 15.12.2021.
//

import SwiftUI

struct EnterPassword: View {
    var content: MEnterPassword
    let pushButton: (Action, String, String) -> Void
    @State private var password: String = ""
    @FocusState private var focused: Bool
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 20.0).foregroundColor(Asset.bg000.swiftUIColor)
            VStack {
                HeaderBar(line1: "SECRET PATH", line2: "///password")
                AddressCard(address: content.authorInfo)
                if content.counter > 0 {
                    Text("Attempt " + String(content.counter) + " of 3")
                }
                ZStack {
                    RoundedRectangle(cornerRadius: 8)
                        .stroke(Asset.crypto400.swiftUIColor)
                        .frame(height: 39)
                    HStack {
                        Text("///").foregroundColor(Asset.crypto400.swiftUIColor)
                        TextField("SECRET PATH", text: $password, prompt: Text(""))
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
                    text: "Next",
                    isCrypto: true,
                    action: {
                        pushButton(.goForward, password, "")
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
