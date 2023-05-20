//
//  PrimaryButton.swift
//  Polkadot Vault
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
