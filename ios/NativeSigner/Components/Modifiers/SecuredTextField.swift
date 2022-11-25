//
//  SecuredTextField.swift
//  NativeSigner
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

    var body: some View {
        ZStack(alignment: .trailing) {
            ZStack {
                SecureField("", text: $text)
                    .focused($focusedField, equals: .secure)
                    .opacity(isSecured ? 1 : 0)
                TextField("", text: $text)
                    .focused($focusedField, equals: .plain)
                    .opacity(isSecured ? 0 : 1)
            }
            .primaryTextFieldStyle(placeholder, text: $text, isValid: $isValid)
            Button(
                action: {
                    isSecured.toggle()
                    focusedField = isSecured ? .secure : .plain
                }
            ) {
                Group {
                    isSecured ? Asset.showPassword.swiftUIImage : Asset.hidePassword.swiftUIImage
                }
                .padding(.trailing, Spacing.medium)
                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
            }
        }
    }
}
