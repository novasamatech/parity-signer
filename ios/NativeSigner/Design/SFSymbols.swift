//
//  SFSymbols.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 04/08/2022.
//

import SwiftUI

/// Base values for Apple's SF Symbols
///
/// Refer to https://developer.apple.com/sf-symbols/ for currently available combinations
enum SFSymbols: String {
    case airplane
    case aqi
    case clear
    case circle
    case checkmark
    case chevron
    case doc
    case ellipsis
    case eye
    case exclamationmark
    case gearshape
    case iphoneArrow = "iphone.and.arrow"
    case lock
    case minus
    case plus
    case rectangle
    case signature
    case square
    case shield
    case trash
    case viewfinder
    case wifi
    case xmark
}


/// Possibly available system variants for `SFSymbol`.
///
/// Refer to https://developer.apple.com/sf-symbols/ for currently available combinations
enum SFSymbolVariant: String {
    case circle
    case down
    case exclamationmark
    case fill
    case forward
    case left
    case grid
    case medium
    case magnifyingglass
    case oneByTwo = "1x2"
    case portrait
    case rectangle
    case shield
    case square
    case slash
    case text
    case triangle
    case trianglebadge
    case viewfinder
}

extension Image {
    init(_ symbol: SFSymbols) {
        self.init(systemName: symbol.rawValue)
    }

    init(_ symbol: SFSymbols, variant: SFSymbolVariant?) {
        self.init(symbol, variants: [variant])
    }

    init(_ symbol: SFSymbols, variants: [SFSymbolVariant?]) {
        let name: String = [[symbol.rawValue], variants.compactMap(\.?.rawValue)]
            .flatMap { $0 }.joined(separator: ".")
        self.init(systemName: name)
    }
}
