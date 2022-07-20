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
            RoundedRectangle(cornerRadius: 20.0).foregroundColor(Color("Bg000"))
            VStack {
                HeaderBar(line1: "Confirm secret path", line2: "Details")
                HStack {
                    Text(content.croppedPath + "///")
                    Image(systemName: "lock").foregroundColor(Color("Crypto400"))
                        .font(FCrypto(style: .body2))
                }
                ZStack {
                    RoundedRectangle(cornerRadius: 8)
                        .stroke(Color("Crypto400"))
                        .frame(height: 39)
                    HStack {
                        Text("///").foregroundColor(Color("Crypto400"))
                        TextField("SECRET PATH", text: $passwordCheck, prompt: Text(""))
                            .foregroundColor(Color("Crypto400"))
                            .font(FCrypto(style: .body2))
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
                        createAddress(content.croppedPath+"///"+content.pwd, content.seedName)
                    },
                    isDisabled: passwordCheck != content.pwd
                )
            }
        }
    }
}

/*
 struct Password_Previews: PreviewProvider {
 static var previews: some View {
 Password()
 }
 }
 */
