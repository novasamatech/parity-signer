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
    private let style: ActionButtonStyle

    init(
        action: @escaping () -> Void,
        text: LocalizedStringKey,
        style: ActionButtonStyle = .primary()
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

// struct PrimaryButton_Previews: PreviewProvider {
//    static var previews: some View {
//        VStack(alignment: .center, spacing: 30) {
//            Text("<< Enabled >>")
//            PrimaryButton(
//                action: {},
//                text: "Short Title"
//            )
//            .padding(10)
//            PrimaryButton(
//                action: {},
//                text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
//            )
//            .padding(10)
//            PrimaryButton(
//                action: {},
//                text: "Short Title",
//                style: .primaryDestructive()
//            )
//            .padding(10)
//            Text("<< Disabled >>")
//            PrimaryButton(
//                action: {},
//                text: "Short Title",
//                style: .primary(isDisabled: Binding<Bool>.constant(true))
//            )
//            .padding(10)
//            PrimaryButton(
//                action: {},
//                text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
//                style: .primary(isDisabled: Binding<Bool>.constant(true))
//            )
//            .padding(10)
//            PrimaryButton(
//                action: {},
//                text: "Short Title",
//                style: .primaryDestructive(isDisabled: Binding<Bool>.constant(true))
//            )
//            .padding(10)
//        }
//        .preferredColorScheme(.dark)
//        .previewLayout(.sizeThatFits)
//        VStack(alignment: .center, spacing: 30) {
//            Text("<< Enabled >>")
//            PrimaryButton(
//                action: {},
//                text: "Short Title"
//            )
//            .padding(10)
//            PrimaryButton(
//                action: {},
//                text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
//            )
//            .padding(10)
//            PrimaryButton(
//                action: {},
//                text: "Short Title",
//                style: .primaryDestructive()
//            )
//            .padding(10)
//            Text("<< Disabled >>")
//            PrimaryButton(
//                action: {},
//                text: "Short Title",
//                style: .primary(isDisabled: Binding<Bool>.constant(true))
//            )
//            .padding(10)
//            PrimaryButton(
//                action: {},
//                text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
//                style: .primary(isDisabled: Binding<Bool>.constant(true))
//            )
//            .padding(10)
//            PrimaryButton(
//                action: {},
//                text: "Short Title",
//                style: .primaryDestructive(isDisabled: Binding<Bool>.constant(true))
//            )
//            .padding(10)
//        }
//        .preferredColorScheme(.light)
//        .previewLayout(.sizeThatFits)
//    }
// }
