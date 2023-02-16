//
//  UnknownNetworkColorsGenerator.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 07/02/2023.
//

import Foundation
import SwiftUI

struct UnknownNetworkRenderable: Equatable, Hashable {
    let background: Color
    let text: Color
}

final class UnknownNetworkColorsGenerator {
    private var mapping: [String: UnknownNetworkRenderable] = [:]

    private var usedColors: Set<UnknownNetworkRenderable> = Set()
    private var remainingColors: Set<UnknownNetworkRenderable> = Color.unknownNetworkBackgrounds

    func renderable(for network: String) -> UnknownNetworkRenderable {
        if let renderable = mapping[network] {
            return renderable
        }
        var uniqueColorFound = false
        repeat {
            if let proposedColor = remainingColors.randomElement() {
                usedColors.insert(proposedColor)
                remainingColors.remove(proposedColor)
                mapping[network] = proposedColor
                uniqueColorFound = true
                return proposedColor
            } else {
                usedColors.removeAll()
                remainingColors = Color.unknownNetworkBackgrounds
            }
        } while !uniqueColorFound
    }
}

private extension Color {
    static let unknownNetworkBackgrounds: Set<UnknownNetworkRenderable> = .init(
        arrayLiteral:
        .init(background: Asset.cyan500.swiftUIColor, text: Asset.accentForegroundTextAlt.swiftUIColor),
        .init(background: Asset.gray500.swiftUIColor, text: Asset.accentForegroundText.swiftUIColor),
        .init(background: Asset.gray600.swiftUIColor, text: Asset.accentForegroundText.swiftUIColor),
        .init(background: Asset.gray800.swiftUIColor, text: Asset.accentForegroundText.swiftUIColor),
        .init(background: Asset.green700.swiftUIColor, text: Asset.accentForegroundTextAlt.swiftUIColor),
        .init(background: Asset.lime600.swiftUIColor, text: Asset.accentForegroundTextAlt.swiftUIColor),
        .init(background: Asset.pink300.swiftUIColor, text: Asset.accentForegroundText.swiftUIColor),
        .init(background: Asset.pink500.swiftUIColor, text: Asset.accentForegroundText.swiftUIColor),
        .init(background: Asset.purple400.swiftUIColor, text: Asset.accentForegroundText.swiftUIColor),
        .init(background: Asset.purple600.swiftUIColor, text: Asset.accentForegroundText.swiftUIColor)
    )
}
