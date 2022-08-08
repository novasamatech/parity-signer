//
//  LogComment.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 3.1.2022.
//

import SwiftUI

struct LogComment: View {
    let navigationRequest: NavigationRequest
    @State private var comment: String = ""
    @FocusState private var focused: Bool
    var body: some View {
        VStack {
            Spacer()
            ZStack {
                RoundedRectangle(cornerRadius: 20.0).foregroundColor(Asset.bg000.swiftUIColor)
                VStack {
                    HeaderBar(line1: "COMMENT", line2: "Enter text")
                    ZStack {
                        RoundedRectangle(cornerRadius: 8)
                            .stroke(Asset.crypto400.swiftUIColor)
                            .frame(height: 39)
                        TextField("COMMENT", text: $comment, prompt: Text(""))
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
                            .onSubmit {
                                navigationRequest(.init(action: .goForward, details: comment))
                            }
                    }
                }
            }
            Spacer()
        }
    }
}

// struct LogComment_Previews: PreviewProvider {
// static var previews: some View {
// LogComment()
// }
// }
