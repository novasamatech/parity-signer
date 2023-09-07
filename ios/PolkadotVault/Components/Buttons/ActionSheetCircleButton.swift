//
//  ActionSheetCircleButton.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 07/09/2023.
//

import SwiftUI

struct ActionSheetCircleButtonStyle: ButtonStyle {
    let foregroundColor: Color

    func makeBody(configuration: Self.Configuration) -> some View {
        configuration.label
            .foregroundColor(foregroundColor)
            .font(PrimaryFont.titleS.font)
    }

    static let destructive = ActionSheetCircleButtonStyle(foregroundColor: Asset.accentRed400.swiftUIColor)
    static let `default` = ActionSheetCircleButtonStyle(foregroundColor: Asset.textAndIconsSecondary.swiftUIColor)
    static let hightlighted = ActionSheetCircleButtonStyle(foregroundColor: Asset.textAndIconsPrimary.swiftUIColor)
}

struct ActionSheetCircleButton: View {
    private let action: () -> Void
    private let icon: Image
    private let text: LocalizedStringKey
    private let style: ActionSheetCircleButtonStyle

    init(
        action: @escaping () -> Void,
        icon: Image,
        text: LocalizedStringKey,
        style: ActionSheetCircleButtonStyle = .default
    ) {
        self.action = action
        self.icon = icon
        self.text = text
        self.style = style
    }

    var body: some View {
        Button(action: action) {
            HStack(alignment: .center, spacing: Spacing.small) {
                icon
                    .frame(width: Sizes.actionSheetCircleIcon, height: Sizes.actionSheetCircleIcon)
                    .background(Circle().foregroundColor(Asset.fill6.swiftUIColor))
                Text(text)
                Spacer()
            }
            .padding(.vertical, Spacing.extraSmall)
            .fixedSize(horizontal: false, vertical: true)
            .contentShape(Rectangle())
        }
        .buttonStyle(style)
    }
}
