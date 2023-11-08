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
                .fill6
            case .error:
                .accentRed300.opacity(0.12)
            case .actionableInfo:
                .accentPink12
            }
        }
    }

    var cornerRadius: CGFloat
    var state: State

    func body(content: Content) -> some View {
        content
            .background(
                RoundedRectangle(cornerRadius: cornerRadius)
                    .stroke(.fill12, lineWidth: 1)
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
