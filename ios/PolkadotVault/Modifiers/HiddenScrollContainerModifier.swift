//
//  HiddenScrollContainerModifier.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 15/09/2022.
//

import SwiftUI

struct HiddenScrollContainerModifier: ViewModifier {
    func body(content: Content) -> some View {
        if #available(iOS 16.0, *) {
            content
                .scrollContentBackground(.hidden)
        } else {
            content
        }
    }
}

extension View {
    /// Hides scroll content background on iOS 16 to match UI for `List` on iOS 15
    func hiddenScrollContent() -> some View {
        modifier(HiddenScrollContainerModifier())
    }
}
