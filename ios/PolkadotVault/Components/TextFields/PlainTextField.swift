//
//  SecuredTextField.swift
//  Polkadot Vault
//
//  Created by Ruslan Rezin on 03/02/2025.
//

import SwiftUI

struct PlainTextFieldStyle: ViewModifier {
    let placeholder: String
    let keyboardType: UIKeyboardType
    @Binding var text: String
    @Binding var isValid: Bool

    func body(content: Content) -> some View {
        content
            .foregroundColor(isValid ? .textAndIconsPrimary : .accentRed300)
            .placeholder(placeholder, when: text.isEmpty)
            .font(PrimaryFont.bodyL.font)
            .autocapitalization(.none)
            .disableAutocorrection(true)
            .keyboardType(keyboardType)
            .submitLabel(.return)
            .frame(height: Heights.textFieldHeight)
    }
}

extension View {
    func plainTextFieldStyle(
        _ placeholder: String,
        keyboardType: UIKeyboardType = .asciiCapable,
        text: Binding<String>,
        isValid: Binding<Bool> = Binding<Bool>.constant(true)
    ) -> some View {
        modifier(
            PlainTextFieldStyle(
                placeholder: placeholder,
                keyboardType: keyboardType,
                text: text,
                isValid: isValid
            )
        )
    }
}
