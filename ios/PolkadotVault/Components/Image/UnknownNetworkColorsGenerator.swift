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
        .init(background: .cyan500, text: .accentForegroundTextAlt),
        .init(background: .gray500, text: .accentForegroundText),
        .init(background: .gray600, text: .accentForegroundText),
        .init(background: .gray800, text: .accentForegroundText),
        .init(background: .green700, text: .accentForegroundTextAlt),
        .init(background: .lime600, text: .accentForegroundTextAlt),
        .init(background: .pink300, text: .accentForegroundText),
        .init(background: .pink500, text: .accentForegroundText),
        .init(background: .purple400, text: .accentForegroundText),
        .init(background: .purple600, text: .accentForegroundText)
    )
}
