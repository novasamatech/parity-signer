//
//  ActionSheetButton.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 05/09/2022.
//

import SwiftUI

struct ActionSheetButtonStyle: ButtonStyle {
    let foregroundColor: Color

    func makeBody(configuration: Self.Configuration) -> some View {
        configuration.label
            .frame(height: Heights.actionSheetButton, alignment: .leading)
            .padding(Spacing.none)
            .foregroundColor(foregroundColor)
            .font(PrimaryFont.labelL.font)
    }

    static let destructive = ActionSheetButtonStyle(foregroundColor: .accentRed400)
    static let `default` = ActionSheetButtonStyle(foregroundColor: .textAndIconsSecondary)
    static let hightlighted = ActionSheetButtonStyle(foregroundColor: .textAndIconsPrimary)
}

struct ActionSheetButton: View {
    private let action: () -> Void
    private let icon: Image
    private let text: LocalizedStringKey
    private let style: ActionSheetButtonStyle

    @State var isDisabled: Bool

    init(
        action: @escaping () -> Void,
        icon: Image,
        text: LocalizedStringKey,
        isDisabled: Bool = false,
        style: ActionSheetButtonStyle = .default
    ) {
        self.action = action
        self.icon = icon
        self.text = text
        self.isDisabled = isDisabled
        self.style = style
    }

    var body: some View {
        Button(action: action) {
            HStack(alignment: .center, spacing: 0) {
                icon
                    .frame(width: Sizes.actionSheetIcon, alignment: .center)
                    .padding(.trailing, Spacing.large)
                Text(text)
                Spacer()
            }
            .frame(maxWidth: .infinity, minHeight: Heights.minimumActionSheetButtonHeight)
            .contentShape(Rectangle())
        }
        .buttonStyle(style)
        .disabled(isDisabled)
    }
}
