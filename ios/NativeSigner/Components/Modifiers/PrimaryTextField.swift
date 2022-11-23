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

    func body(content: Content) -> some View {
        content
            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
            .placeholder(placeholder, when: text.isEmpty)
            .font(Fontstyle.bodyL.base)
            .autocapitalization(.none)
            .keyboardType(.asciiCapable)
            .submitLabel(.done)
            .frame(height: Heights.textFieldHeight)
            .padding(.horizontal, Spacing.medium)
            .background(Asset.fill6.swiftUIColor)
            .cornerRadius(CornerRadius.small)
    }
}

extension View {
    func primaryTextFieldStyle(_ placeholder: String, text: Binding<String>) -> some View {
        modifier(PrimaryTextFieldStyle(placeholder: placeholder, text: text))
    }
}
