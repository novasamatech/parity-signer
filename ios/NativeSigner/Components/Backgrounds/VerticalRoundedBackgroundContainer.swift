//
//  VerticalRoundedBackgroundContainer.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 21/12/2022.
//

import SwiftUI

struct VerticalRoundedBackgroundContainer: ViewModifier {
    var cornerRadius: CGFloat

    func body(content: Content) -> some View {
        VStack {
            content
                .padding(Spacing.medium)
        }
        .background(Asset.fill6Solid.swiftUIColor)
        .cornerRadius(cornerRadius)
    }
}

extension View {
    func verticalRoundedBackgroundContainer(_ cornerRadius: CGFloat = CornerRadius.medium) -> some View {
        modifier(VerticalRoundedBackgroundContainer(cornerRadius: cornerRadius))
    }
}
