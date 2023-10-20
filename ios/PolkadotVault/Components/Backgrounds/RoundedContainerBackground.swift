//
//  RoundedContainerBackground.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 24/11/2022.
//

import SwiftUI

struct RoundedContainerBackground: ViewModifier {
    let cornerRadius: CGFloat
    let state: State

    enum State {
        case list
        case standard
        case textContainer
        case error
        case actionableInfo

        var foregroundColor: Color {
            switch self {
            case .list:
                .backgroundSecondary
            case .standard:
                .fill6
            case .textContainer:
                .fill12
            case .error:
                .accentRed300.opacity(0.12)
            case .actionableInfo:
                .accentPink12
            }
        }
    }

    func body(content: Content) -> some View {
        content
            .background(
                RoundedRectangle(cornerRadius: cornerRadius)
                    .foregroundColor(state.foregroundColor)
            )
    }
}

extension View {
    func containerBackground(
        _ cornerRadius: CGFloat = CornerRadius.medium,
        state: RoundedContainerBackground.State = .standard
    ) -> some View {
        modifier(RoundedContainerBackground(cornerRadius: cornerRadius, state: state))
    }
}
