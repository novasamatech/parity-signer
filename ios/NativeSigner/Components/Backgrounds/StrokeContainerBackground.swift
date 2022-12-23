//
//  StrokeContainerBackground.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 22/11/2022.
//

import SwiftUI

struct StrokeContainerBackground: ViewModifier {
    var cornerRadius: CGFloat
    var isError: Bool

    func body(content: Content) -> some View {
        content
            .background(
                RoundedRectangle(cornerRadius: cornerRadius)
                    .stroke(Asset.fill12.swiftUIColor, lineWidth: 1)
                    .background(isError ? Asset.accentRed300.swiftUIColor.opacity(0.12) : Asset.fill6.swiftUIColor)
                    .cornerRadius(cornerRadius)
            )
    }
}

extension View {
    func strokeContainerBackground(_ cornerRadius: CGFloat = CornerRadius.medium, isError: Bool = false) -> some View {
        modifier(StrokeContainerBackground(cornerRadius: cornerRadius, isError: isError))
    }
}
