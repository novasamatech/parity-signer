//
//  LogComment.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 3.1.2022.
//

import SwiftUI

struct LogComment: View {
    let pushButton: (Action, String, String) -> Void
    @State private var comment: String = ""
    @FocusState private var focused: Bool
    var body: some View {
        VStack {
            Spacer()
            ZStack {
                RoundedRectangle(cornerRadius: 20.0).foregroundColor(Color("Bg000"))
                VStack {
                    HeaderBar(line1: "COMMENT", line2: "Enter text")
                    ZStack {
                        RoundedRectangle(cornerRadius: 8)
                            .stroke(Color("Crypto400"))
                            .frame(height: 39)
                        TextField("COMMENT", text: $comment, prompt: Text(""))
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
                            .onSubmit {
                                pushButton(.goForward, comment, "")
                            }
                    }
                }
            }
            Spacer()
        }
    }
}

/*
 struct LogComment_Previews: PreviewProvider {
 static var previews: some View {
 LogComment()
 }
 }
 */
