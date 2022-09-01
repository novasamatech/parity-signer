//
//  ActionButton.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 23/08/2022.
//

import SwiftUI

struct ActionButtonStyle: ButtonStyle {
    let backgroundColor: Color
    let foregroundColor: Color

    func makeBody(configuration: Self.Configuration) -> some View {
        configuration.label
            .padding(Spacing.large)
            .background(backgroundColor)
            .foregroundColor(foregroundColor)
            .cornerRadius(CornerRadius.extraExtraLarge)
            .font(Fontstyle.labelL.base)
            .frame(height: Heights.actionButton, alignment: .center)
    }
}

struct ActionButton: View {
    private let action: () -> Void
    private let text: LocalizedStringKey
    private let style: ActionButtonStyle

    @Binding var isDisabled: Bool

    init(
        action: @escaping () -> Void,
        text: LocalizedStringKey,
        style: ActionButtonStyle,
        isDisabled: Binding<Bool> = Binding<Bool>.constant(false)
    ) {
        self.action = action
        self.text = text
        self.style = style
        _isDisabled = isDisabled
    }

    var body: some View {
        Button(action: action) {
            HStack {
                Text(text)
            }
            .frame(maxWidth: .infinity)
        }
        .buttonStyle(style)
        .disabled(isDisabled)
    }
}
