//
//  SecuredTextField.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 24/11/2022.
//

import SwiftUI

struct SecurePrimaryTextField: View {
    enum Field: Hashable {
        case secure
        case plain
    }

    @State private var isSecured: Bool = true

    let placeholder: String
    @Binding var text: String
    @Binding var isValid: Bool
    @FocusState var focusedField: Field?
    var onCommit: (() -> Void) = {}

    var body: some View {
			HStack(alignment: .center, spacing: Spacing.minimal) {
            ZStack {
                SecureField("", text: $text, onCommit: {
                    onCommit()
                })
                .autocorrectionDisabled()
                .transaction { $0.animation = nil }
                .focused($focusedField, equals: .secure)
                .opacity(isSecured ? 1 : 0)
                TextField("", text: $text, onCommit: {
                    onCommit()
                })
                .autocorrectionDisabled()
                .transaction { $0.animation = nil }
                .focused($focusedField, equals: .plain)
                .opacity(isSecured ? 0 : 1)
            }
						.plainTextFieldStyle(placeholder, text: $text, isValid: $isValid)
            .onChange(of: text) { _ in
                isValid = true
            }
            Button(
                action: {
                    isSecured.toggle()
                    focusedField = isSecured ? .secure : .plain
                }
            ) {
                Image(isSecured ? .showPassword : .hidePassword)
                    .foregroundColor(.textAndIconsTertiary)
            }
        }
        .padding(.horizontal, Spacing.medium)
        .containerBackground(CornerRadius.small, state: isValid ? .standard : .error)
    }
}
