//
//  EnterPassword.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 15.12.2021.
//

import SwiftUI

struct EnterPassword: View {
    @EnvironmentObject var data: SignerDataModel
    var content: MEnterPassword
    @State private var password: String = ""
    @FocusState private var focused: Bool
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 20.0).foregroundColor(Color("Bg000"))
            VStack {
                HeaderBar(line1: "SECRET PATH", line2: "///password")
                AddressCard(address: content.author_info.intoAddress())
                if (content.counter>0) {
                    Text("Attempt " + String(content.counter) + " of 3")
                }
                ZStack {
                    RoundedRectangle(cornerRadius: 8)
                        .stroke(Color("Crypto400"))
                        .frame(height: 39)
                    HStack {
                        Text("///").foregroundColor(Color("Crypto400"))
                    TextField("SECRET PATH", text: $password, prompt: Text(""))
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
                        .onAppear {
                            focused = true
                        }
                    }
                }
                Button(action: {
                    data.pushButton(buttonID: .GoForward, details: password)
                }) {
                    Text("Next")
                }.disabled(password != "")
            }
        }
    }
}

/*
struct EnterPassword_Previews: PreviewProvider {
    static var previews: some View {
        EnterPassword()
    }
}
*/
