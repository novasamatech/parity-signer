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
    @StateObject var keyboardOffsetAdapter = KeyboardOffsetAdapter()
    @State private var password: String = ""
    @FocusState private var focused: Bool
    @State var animateBackground: Bool = false

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: {
                dismiss()
            },
            animateBackground: $animateBackground
        ) {
            VStack(spacing: Spacing.medium) {
                HeaderBar(line1: Localizable.secretPath.key, line2: Localizable.Path.password.key)
                AddressCard(card: content.authorInfo)
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
                BigButton(
                    text: "Cancel",
                    isShaded: true,
                    isDangerous: true,
                    action: {
                        focused = false
                        navigationRequest(.init(action: .goBack))
                    }
                )
            }
            .onAppear {
                focused = true
            }
            .padding(Spacing.medium)
            .cornerRadius(CornerRadius.extraSmall)
            .background(Asset.bg000.swiftUIColor)
            .padding(.bottom, keyboardOffsetAdapter.keyboardHeight)
        }
    }

    func dismiss() {
        navigationRequest(.init(action: .goBack))
    }
}

struct EnterPassword_Previews: PreviewProvider {
    static var previews: some View {
        EnterPassword(
            content: MEnterPassword(authorInfo: .init(
                base58: "fdsfsf",
                address: .init(
                    path: "path",
                    hasPwd: true,
                    identicon: PreviewData.exampleIdenticon,
                    seedName: "password",
                    secretExposed: true
                ),
                multiselect: nil
            ), counter: 2),
            navigationRequest: { _ in }
        )
    }
}
