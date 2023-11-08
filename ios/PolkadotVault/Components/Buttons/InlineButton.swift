//
//  InlineButton.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 23/11/2022.
//

import SwiftUI

struct InlineButtonStyle: ButtonStyle {
    let foregroundColor: Color

    func makeBody(configuration: Self.Configuration) -> some View {
        configuration.label
            .padding(Spacing.none)
            .foregroundColor(foregroundColor)
            .font(PrimaryFont.bodyL.font)
            .frame(height: Heights.menuButton, alignment: .leading)
    }
}

struct InlineButton: View {
    private let action: () -> Void
    private let icon: Image
    private let text: String

    init(
        action: @escaping () -> Void,
        icon: Image,
        text: String
    ) {
        self.action = action
        self.icon = icon
        self.text = text
    }

    var body: some View {
        Button(action: action) {
            HStack(alignment: .center, spacing: Spacing.extraExtraSmall) {
                icon
                    .frame(width: Sizes.actionSheetIcon, alignment: .center)
                Text(text)
                Spacer()
            }
            .frame(maxWidth: .infinity, alignment: .leading)
        }
        .buttonStyle(InlineButtonStyle(foregroundColor: .textAndIconsPrimary))
    }
}

#if DEBUG
    struct InlineButton_Previews: PreviewProvider {
        static var previews: some View {
            VStack(alignment: .leading, spacing: 10) {
                InlineButton(
                    action: {},
                    icon: Image(.addLarge),
                    text: Localizable.TransactionSign.Action.note.string
                )
                .padding(10)
            }
            .preferredColorScheme(.dark)
            .previewLayout(.sizeThatFits)
            VStack(alignment: .leading, spacing: 10) {
                InlineButton(
                    action: {},
                    icon: Image(.addLarge),
                    text: Localizable.TransactionSign.Action.note.string
                )
                .padding(10)
            }
            .preferredColorScheme(.light)
            .previewLayout(.sizeThatFits)
        }
    }
#endif
