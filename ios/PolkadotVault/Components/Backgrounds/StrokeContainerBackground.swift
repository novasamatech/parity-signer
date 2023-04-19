//
//  StrokeContainerBackground.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 22/11/2022.
//

import SwiftUI

struct StrokeContainerBackground: ViewModifier {
    enum State {
        case standard
        case error
        case actionableInfo

        var backgroundColor: Color {
            switch self {
            case .standard:
                return Asset.fill6.swiftUIColor
            case .error:
                return Asset.accentRed300.swiftUIColor.opacity(0.12)
            case .actionableInfo:
                return Asset.accentPink12.swiftUIColor
            }
        }
    }

    var cornerRadius: CGFloat
    var state: State

    func body(content: Content) -> some View {
        content
            .background(
                RoundedRectangle(cornerRadius: cornerRadius)
                    .stroke(Asset.fill12.swiftUIColor, lineWidth: 1)
                    .background(state.backgroundColor)
                    .cornerRadius(cornerRadius)
            )
    }
}

extension View {
    func strokeContainerBackground(
        _ cornerRadius: CGFloat = CornerRadius.medium,
        state: StrokeContainerBackground.State = .standard
    ) -> some View {
        modifier(StrokeContainerBackground(cornerRadius: cornerRadius, state: state))
    }
}
