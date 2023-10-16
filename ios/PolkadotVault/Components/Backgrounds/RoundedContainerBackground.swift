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
                Asset.backgroundSecondary.swiftUIColor
            case .standard:
                Asset.fill6.swiftUIColor
            case .textContainer:
                Asset.fill12.swiftUIColor
            case .error:
                Asset.accentRed300.swiftUIColor.opacity(0.12)
            case .actionableInfo:
                Asset.accentPink12.swiftUIColor
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
