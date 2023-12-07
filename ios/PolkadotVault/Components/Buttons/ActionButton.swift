//
//  ActionButton.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 23/08/2022.
//

import SwiftUI

struct ActionButtonStyle: ButtonStyle {
    let backgroundColor: Color
    let foregroundColor: Color
    @Binding var isDisabled: Bool

    func makeBody(configuration: Self.Configuration) -> some View {
        configuration.label
            .padding(.vertical, Spacing.large)
            .background(backgroundColor)
            .foregroundColor(foregroundColor)
            .frame(height: Heights.actionButton, alignment: .center)
            .cornerRadius(CornerRadius.extraExtraLarge)
            .font(PrimaryFont.labelL.font)
            .disabled(isDisabled)
    }

    static func primary(isDisabled: Binding<Bool> = Binding<Bool>.constant(false)) -> ActionButtonStyle {
        ActionButtonStyle(
            backgroundColor: isDisabled.wrappedValue ? .accentPink500Disabled : .accentPink500,
            foregroundColor: isDisabled.wrappedValue ? .accentPink500TextDisabled : .accentForegroundText,
            isDisabled: isDisabled
        )
    }

    static func primaryDestructive(isDisabled: Binding<Bool> = Binding<Bool>.constant(false)) -> ActionButtonStyle {
        ActionButtonStyle(
            backgroundColor: .accentRed400,
            foregroundColor: isDisabled.wrappedValue ? .accentForegroundTextDisabled : .accentForegroundText,

            isDisabled: isDisabled
        )
    }

    static func secondary(isDisabled: Binding<Bool> = Binding<Bool>.constant(false)) -> ActionButtonStyle {
        ActionButtonStyle(
            backgroundColor: .fill18,
            foregroundColor: isDisabled.wrappedValue ? .textAndIconsDisabled : .textAndIconsPrimary,
            isDisabled: isDisabled
        )
    }

    static func emptyPrimary(isDisabled: Binding<Bool> = Binding<Bool>.constant(false)) -> ActionButtonStyle {
        ActionButtonStyle(
            backgroundColor: .clear,
            foregroundColor: isDisabled.wrappedValue ? .textAndIconsDisabled : .textAndIconsPrimary,
            isDisabled: isDisabled
        )
    }

    static func emptySecondary(isDisabled: Binding<Bool> = Binding<Bool>.constant(false)) -> ActionButtonStyle {
        ActionButtonStyle(
            backgroundColor: .clear,
            foregroundColor: .textAndIconsSecondary,
            isDisabled: isDisabled
        )
    }
}

struct ActionButton: View {
    private let action: () -> Void
    private let text: LocalizedStringKey
    private var style: ActionButtonStyle

    init(
        action: @escaping () -> Void,
        text: LocalizedStringKey,
        style: ActionButtonStyle
    ) {
        self.action = action
        self.text = text
        self.style = style
    }

    var body: some View {
        Button(action: action) {
            HStack {
                Text(text)
            }
            .frame(maxWidth: .infinity)
        }
        .buttonStyle(style)
        .disabled(style.isDisabled)
    }
}
