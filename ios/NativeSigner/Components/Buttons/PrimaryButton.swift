//
//  PrimaryButton.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 19/08/2022.
//

import SwiftUI

struct PrimaryButton: View {
    var action: () -> Void
    var text: LocalizedStringKey
    @State var isDisabled: Bool = false

    var body: some View {
        ActionButton(
            action: action,
            text: text,
            style: ActionButtonStyle(
                backgroundColor: Asset.accentPink500.swiftUIColor,
                foregroundColor: (isDisabled ? Asset.accentForegroundTextDisabled : Asset.accentForegroundText)
                    .swiftUIColor
            ),
            isDisabled: isDisabled
        )
    }
}

struct ActionButton: View {
    var action: () -> Void
    var text: LocalizedStringKey
    var style: ActionButtonStyle
    @State var isDisabled: Bool = false

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

struct PrimaryButton_Previews: PreviewProvider {
    static var previews: some View {
        VStack {
            Text("<< Enabled >>")
            PrimaryButton(
                action: {},
                text: "Short Title"
            )
            PrimaryButton(
                action: {},
                text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
            )
            Text("<< Disabled >>")
            PrimaryButton(
                action: {},
                text: "Short Title",
                isDisabled: true
            )
            PrimaryButton(
                action: {},
                text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                isDisabled: true
            )
        }
        .preferredColorScheme(.dark)
        .previewLayout(.sizeThatFits)
    }
}
