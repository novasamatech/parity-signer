//
//  View+Placeholder.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 23/11/2022.
//

import SwiftUI

extension View {
    func placeholder(
        _ text: String,
        when shouldShow: Bool,
        alignment: Alignment = .leading
    ) -> some View {
        placeholder(when: shouldShow, alignment: alignment) {
            Text(text)
                .foregroundColor(.textAndIconsTertiary)
        }
    }

    private func placeholder(
        when shouldShow: Bool,
        alignment: Alignment = .leading,
        @ViewBuilder placeholder: () -> some View
    ) -> some View {
        ZStack(alignment: alignment) {
            placeholder().opacity(shouldShow ? 1 : 0)
            self
        }
    }
}
