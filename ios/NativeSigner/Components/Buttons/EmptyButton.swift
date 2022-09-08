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
    private let style: ActionButtonStyle

    init(
        action: @escaping () -> Void,
        text: LocalizedStringKey,
        style: ActionButtonStyle = .emptyPrimary()
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

//
// struct EmptyButton_Previews: PreviewProvider {
//    static var previews: some View {
//        VStack(alignment: .center, spacing: 10) {
//            Text("<< Enabled >>")
//            EmptyButton(
//                action: {},
//                text: "Short Title"
//            )
//            .padding(10)
//            EmptyButton(
//                action: {},
//                text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
//            )
//            .padding(10)
//            Text("<< Disabled >>")
//            EmptyButton(
//                action: {},
//                text: "Short Title",
//                style: .emptyPrimary(isDisabled: Binding<Bool>.constant(true))
//            )
//            .padding(10)
//            EmptyButton(
//                action: {},
//                text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
//                style: .emptyPrimary(isDisabled: Binding<Bool>.constant(true))
//            )
//            .padding(10)
//        }
//        .preferredColorScheme(.dark)
//        .previewLayout(.sizeThatFits)
//        VStack(alignment: .center, spacing: 10) {
//            Text("<< Enabled >>")
//            EmptyButton(
//                action: {},
//                text: "Short Title"
//            )
//            .padding(10)
//            EmptyButton(
//                action: {},
//                text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
//            )
//            .padding(10)
//            Text("<< Disabled >>")
//            EmptyButton(
//                action: {},
//                text: "Short Title",
//                style: .emptyPrimary(isDisabled: Binding<Bool>.constant(true))
//            )
//            .padding(10)
//            EmptyButton(
//                action: {},
//                text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
//                style: .emptyPrimary(isDisabled: Binding<Bool>.constant(true))
//            )
//            .padding(10)
//        }
//        .preferredColorScheme(.light)
//        .previewLayout(.sizeThatFits)
//    }
// }
