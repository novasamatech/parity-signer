//
//  InlineTextField.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 23/11/2022.
//

import SwiftUI

struct InlineTextFieldStyle: ViewModifier {
    @Binding var text: String

    func body(content: Content) -> some View {
        content
            .foregroundColor(.textAndIconsPrimary)
            .font(PrimaryFont.bodyM.font)
            .autocapitalization(.none)
            .disableAutocorrection(true)
            .keyboardType(.asciiCapable)
            .submitLabel(.done)
            .frame(height: Heights.seedPhraseCapsuleHeight)
            .padding(.horizontal, Spacing.small)
    }
}

extension View {
    func inlineTextFieldStyle(
        text: Binding<String>
    ) -> some View {
        modifier(InlineTextFieldStyle(text: text))
    }
}
