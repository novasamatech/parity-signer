//
//  RoundedContainerBackground.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 24/11/2022.
//

import SwiftUI

struct RoundedContainerBackground: ViewModifier {
    var cornerRadius: CGFloat
    var isTinted: Bool

    func body(content: Content) -> some View {
        content
            .background(
                RoundedRectangle(cornerRadius: cornerRadius)
                    .foregroundColor(
                        isTinted ? Asset.accentRed300.swiftUIColor.opacity(0.12) : Asset.fill6
                            .swiftUIColor
                    )
            )
    }
}

extension View {
    func containerBackground(_ cornerRadius: CGFloat = CornerRadius.medium, isTinted: Bool = false) -> some View {
        modifier(RoundedContainerBackground(cornerRadius: cornerRadius, isTinted: isTinted))
    }
}
