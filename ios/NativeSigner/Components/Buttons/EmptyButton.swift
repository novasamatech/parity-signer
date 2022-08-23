//
//  EmptyButton.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 19/08/2022.
//

import SwiftUI

struct EmptyButton: View {
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
                backgroundColor: .clear,
                foregroundColor: Asset.textAndIconsPrimary.swiftUIColor
            ),
            isDisabled: $isDisabled
        )
    }
}

struct EmptyButton_Previews: PreviewProvider {
    static var previews: some View {
        VStack {
            Text("<< Enabled >>")
            EmptyButton(
                action: {},
                text: "Short Title"
            )
            EmptyButton(
                action: {},
                text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
            )
            Text("<< Disabled >>")
            EmptyButton(
                action: {},
                text: "Short Title",
                isDisabled: true
            )
            EmptyButton(
                action: {},
                text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                isDisabled: true
            )
        }
        .preferredColorScheme(.dark)
        .previewLayout(.sizeThatFits)
    }
}
