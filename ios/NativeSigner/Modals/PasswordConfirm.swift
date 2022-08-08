//
//  Password.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 13.12.2021.
//

import SwiftUI

struct PasswordConfirm: View {
    var content: MPasswordConfirm
    let createAddress: (String, String) -> Void
    @State private var passwordCheck: String = ""
    @FocusState private var focused: Bool

    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 20.0).foregroundColor(Asset.bg000.swiftUIColor)
            VStack {
                HeaderBar(line1: "Confirm secret path", line2: "Details")
                HStack {
                    Text(content.croppedPath + "///")
                    Image(.lock).foregroundColor(Asset.crypto400.swiftUIColor)
                        .font(Fontstyle.body2.crypto)
                }
                ZStack {
                    RoundedRectangle(cornerRadius: 8)
                        .stroke(Asset.crypto400.swiftUIColor)
                        .frame(height: 39)
                    HStack {
                        Text("///").foregroundColor(Asset.crypto400.swiftUIColor)
                        TextField("SECRET PATH", text: $passwordCheck, prompt: Text(""))
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
                    action: {
                        createAddress(content.croppedPath + "///" + content.pwd, content.seedName)
                    },
                    isDisabled: passwordCheck != content.pwd
                )
            }
        }
    }
}

// struct Password_Previews: PreviewProvider {
// static var previews: some View {
// Password()
// }
// }
