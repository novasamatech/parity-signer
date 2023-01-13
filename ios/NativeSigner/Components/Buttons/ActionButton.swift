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
    @Binding var isDisabled: Bool

    func makeBody(configuration: Self.Configuration) -> some View {
        configuration.label
            .padding(Spacing.large)
            .background(backgroundColor)
            .foregroundColor(foregroundColor)
            .frame(height: Heights.actionButton, alignment: .center)
            .cornerRadius(CornerRadius.extraExtraLarge)
            .font(PrimaryFont.labelL.font)
            .disabled(isDisabled)
    }

    static func primary(isDisabled: Binding<Bool> = Binding<Bool>.constant(false)) -> ActionButtonStyle {
        ActionButtonStyle(
            backgroundColor: isDisabled.wrappedValue ? Asset.accentPink500Disabled.swiftUIColor : Asset.accentPink500
                .swiftUIColor,
            foregroundColor: isDisabled.wrappedValue ? Asset.accentPink500TextDisabled.swiftUIColor : Asset
                .accentForegroundText
                .swiftUIColor,
            isDisabled: isDisabled
        )
    }

    static func primaryDestructive(isDisabled: Binding<Bool> = Binding<Bool>.constant(false)) -> ActionButtonStyle {
        ActionButtonStyle(
            backgroundColor: Asset.accentRed400.swiftUIColor,
            foregroundColor: (isDisabled.wrappedValue ? Asset.accentForegroundTextDisabled : Asset.accentForegroundText)
                .swiftUIColor,
            isDisabled: isDisabled
        )
    }

    static func secondary(isDisabled: Binding<Bool> = Binding<Bool>.constant(false)) -> ActionButtonStyle {
        ActionButtonStyle(
            backgroundColor: Asset.fill18.swiftUIColor,
            foregroundColor: (isDisabled.wrappedValue ? Asset.textAndIconsDisabled : Asset.textAndIconsPrimary)
                .swiftUIColor,
            isDisabled: isDisabled
        )
    }

    static func emptyPrimary(isDisabled: Binding<Bool> = Binding<Bool>.constant(false)) -> ActionButtonStyle {
        ActionButtonStyle(
            backgroundColor: .clear,
            foregroundColor: Asset.textAndIconsPrimary.swiftUIColor,
            isDisabled: isDisabled
        )
    }

    static func emptySecondary(isDisabled: Binding<Bool> = Binding<Bool>.constant(false)) -> ActionButtonStyle {
        ActionButtonStyle(
            backgroundColor: .clear,
            foregroundColor: Asset.textAndIconsSecondary.swiftUIColor,
            isDisabled: isDisabled
        )
    }
}

struct ActionButton: View {
    private let action: () -> Void
    private let text: LocalizedStringKey
    private var style: ActionButtonStyle

    init(
        action: @escaping @autoclosure () -> Void,
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
