//
//  Password.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 13.12.2021.
//

import SwiftUI

struct PasswordConfirm: View {
    @EnvironmentObject var data: SignerDataModel
    var content: MPasswordConfirm
    @State private var passwordCheck: String = "///"
    @FocusState private var focused: Bool
    
    var body: some View {
        VStack {
            HeaderBar(line1: "Confirm secret path", line2: "Details")
            ZStack {
                RoundedRectangle(cornerRadius: 8).stroke(Color("AccentColor")).foregroundColor(Color("backgroundColor")).frame(height: 39)
                TextField("SECRET PATH", text: $passwordCheck, prompt: Text("///"))
                    .foregroundColor(Color("Crypto400"))
                    .font(FCrypto(style: .body2))
                    .disableAutocorrection(true)
                    .autocapitalization(.none)
                    .keyboardType(.asciiCapable)
                    .submitLabel(.done)
                    .onChange(of: data.suggestedName, perform: {_ in data.lastError = ""
                    })
                    .focused($focused)
                    .padding(8)
            }
            Button(action: {}) {
                Text("Next")
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
