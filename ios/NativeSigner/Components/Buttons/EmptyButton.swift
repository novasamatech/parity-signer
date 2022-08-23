//
//  EmptyButton.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 19/08/2022.
//

import SwiftUI

struct EmptyButton: View {
    var action: () -> Void
    var text: LocalizedStringKey
    @State var isDisabled: Bool = false

    var body: some View {
        Button(action: action) {
            HStack {
                Text(text)
            }
        }
        .buttonStyle(
            ActionButtonStyle(
                backgroundColor: .clear,
                foregroundColor: Asset.textAndIconsPrimary.swiftUIColor
            )
        )
        .disabled(isDisabled)
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
