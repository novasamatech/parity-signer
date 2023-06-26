//
//  EmptyButton.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 19/08/2022.
//

import SwiftUI

struct EmptyButton: View {
    private let action: () -> Void
    private let text: LocalizedStringKey
    private let style: ActionButtonStyle

    init(
        action: @escaping @autoclosure () -> Void,
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
