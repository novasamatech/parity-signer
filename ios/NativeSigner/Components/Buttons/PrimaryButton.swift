//
//  PrimaryButton.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 19/08/2022.
//

import SwiftUI

struct PrimaryButton: View {
    private let action: () -> Void
    private let text: LocalizedStringKey

    @State var isDisabled: Bool

    init(
        action: @escaping () -> Void,
        text: LocalizedStringKey,
        isDisabled: Bool = false
    ) {
        self.action = action
        self.text = text
        self.isDisabled = isDisabled
    }

    var body: some View {
        ActionButton(
            action: action,
            text: text,
            style: ActionButtonStyle(
                backgroundColor: Asset.accentPink500.swiftUIColor,
                foregroundColor: (isDisabled ? Asset.accentForegroundTextDisabled : Asset.accentForegroundText)
                    .swiftUIColor
            ),
            isDisabled: $isDisabled
        )
    }
}

struct PrimaryButton_Previews: PreviewProvider {
    static var previews: some View {
        VStack(alignment: .center, spacing: 30) {
            Text("<< Enabled >>")
            PrimaryButton(
                action: {},
                text: "Short Title"
            )
            .padding(10)
            PrimaryButton(
                action: {},
                text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
            )
            .padding(10)
            Text("<< Disabled >>")
            PrimaryButton(
                action: {},
                text: "Short Title",
                isDisabled: true
            )
            .padding(10)
            PrimaryButton(
                action: {},
                text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                isDisabled: true
            )
            .padding(10)
        }
        .preferredColorScheme(.dark)
        .previewLayout(.sizeThatFits)
        VStack(alignment: .center, spacing: 30) {
            Text("<< Enabled >>")
            PrimaryButton(
                action: {},
                text: "Short Title"
            )
            .padding(10)
            PrimaryButton(
                action: {},
                text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
            )
            .padding(10)
            Text("<< Disabled >>")
            PrimaryButton(
                action: {},
                text: "Short Title",
                isDisabled: true
            )
            .padding(10)
            PrimaryButton(
                action: {},
                text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                isDisabled: true
            )
            .padding(10)
        }
        .preferredColorScheme(.light)
        .previewLayout(.sizeThatFits)
    }
}
