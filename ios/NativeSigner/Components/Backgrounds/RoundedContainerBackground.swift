//
//  RoundedContainerBackground.swift
//  NativeSigner
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
        case error
        case actionableInfo

        var foregroundColor: Color {
            switch self {
            case .list:
                return Asset.backgroundSecondary.swiftUIColor
            case .standard:
                return Asset.fill6.swiftUIColor
            case .error:
                return Asset.accentRed300.swiftUIColor.opacity(0.12)
            case .actionableInfo:
                return Asset.accentPink12.swiftUIColor
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
