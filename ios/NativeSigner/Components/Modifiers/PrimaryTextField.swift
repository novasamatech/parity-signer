//
//  PrimaryTextField.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 23/11/2022.
//

import SwiftUI

struct PrimaryTextFieldStyle: ViewModifier {
    let placeholder: String
    @Binding var text: String
    @Binding var isValid: Bool

    func body(content: Content) -> some View {
        content
            .foregroundColor(isValid ? Asset.textAndIconsPrimary.swiftUIColor : Asset.accentRed300.swiftUIColor)
            .placeholder(placeholder, when: text.isEmpty)
            .font(PrimaryFont.bodyL.font)
            .autocapitalization(.none)
            .keyboardType(.asciiCapable)
            .submitLabel(.return)
            .frame(height: Heights.textFieldHeight)
            .padding(.horizontal, Spacing.medium)
            .background(Asset.fill6.swiftUIColor)
            .cornerRadius(CornerRadius.small)
            .overlay(
                RoundedRectangle(cornerRadius: CornerRadius.small)
                    .stroke(isValid ? .clear : Asset.accentRed300.swiftUIColor, lineWidth: 1)
            )
    }
}

extension View {
    func primaryTextFieldStyle(
        _ placeholder: String,
        text: Binding<String>,
        isValid: Binding<Bool> = Binding<Bool>.constant(true)
    ) -> some View {
        modifier(PrimaryTextFieldStyle(placeholder: placeholder, text: text, isValid: isValid))
    }
}
