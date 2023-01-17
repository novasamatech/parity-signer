//
//  Password.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 13.12.2021.
//

import SwiftUI

struct PasswordConfirm: View {
    var content: MPasswordConfirm
    @EnvironmentObject var data: SignerDataModel
    @State private var passwordCheck: String = ""
    @FocusState private var focused: Bool

    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 20.0).foregroundColor(Asset.backgroundPrimary.swiftUIColor)
            VStack {
                HeaderBar(line1: Localizable.confirmSecretPath.key, line2: Localizable.details.key)
                HStack {
                    Text(content.croppedPath + Localizable.Path.delimeter.string)
                    Image(.lock).foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .font(PrimaryFont.captionM.font)
                }
                ZStack {
                    RoundedRectangle(cornerRadius: 8)
                        .stroke(Asset.fill6.swiftUIColor)
                        .frame(height: 39)
                    HStack {
                        Localizable.Path.delimeter.text
                            .foregroundColor(Asset.accentPink300.swiftUIColor)
                        TextField(Localizable.secretPath.string, text: $passwordCheck, prompt: Text(""))
                            .foregroundColor(Asset.accentPink300.swiftUIColor)
                            .font(PrimaryFont.captionM.font)
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
                PrimaryButton(
                    action: {
                        data.createAddress(
                            path: content.croppedPath + Localizable.Path.delimeter.string + content.pwd,
                            seedName: content.seedName
                        )
                    },
                    text: Localizable.next.key,
                    style: .primary(isDisabled: .constant(passwordCheck != content.pwd))
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
