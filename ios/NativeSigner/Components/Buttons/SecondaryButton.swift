//
//  SecondaryButton.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 19/08/2022.
//

import SwiftUI

struct SecondaryButton: View {
    private let action: () -> Void
    private let text: LocalizedStringKey
    private let style: ActionButtonStyle

    init(
        action: @escaping @autoclosure () -> Void,
        text: LocalizedStringKey,
        style: ActionButtonStyle = .secondary()
    ) {
        self.action = action
        self.text = text
        self.style = style
    }

    var body: some View {
        ActionButton(
            action: action(),
            text: text,
            style: style
        )
    }
}

struct SecondaryButton_Previews: PreviewProvider {
    static var previews: some View {
        VStack(alignment: .center, spacing: 10) {
            Text("<< Enabled >>")
            SecondaryButton(
                action: {}(),
                text: "Short Title"
            )
            .padding(10)
            SecondaryButton(
                action: {}(),
                text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
            )
            .padding(10)
            Text("<< Disabled >>")
            SecondaryButton(
                action: {}(),
                text: "Short Title",
                style: .secondary(isDisabled: Binding<Bool>.constant(true))
            )
            .padding(10)
            SecondaryButton(
                action: {}(),
                text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                style: .secondary(isDisabled: Binding<Bool>.constant(true))
            )
            .padding(10)
        }
        .preferredColorScheme(.dark)
        .previewLayout(.sizeThatFits)
        VStack(alignment: .center, spacing: 10) {
            Text("<< Enabled >>")
            SecondaryButton(
                action: {}(),
                text: "Short Title"
            )
            .padding(10)
            SecondaryButton(
                action: {}(),
                text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
            )
            .padding(10)
            Text("<< Disabled >>")
            SecondaryButton(
                action: {}(),
                text: "Short Title",
                style: .secondary(isDisabled: Binding<Bool>.constant(true))
            )
            .padding(10)
            SecondaryButton(
                action: {}(),
                text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                style: .secondary(isDisabled: Binding<Bool>.constant(true))
            )
            .padding(10)
        }
        .preferredColorScheme(.light)
        .previewLayout(.sizeThatFits)
    }
}
