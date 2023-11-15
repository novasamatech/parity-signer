//
//  SecondaryFont.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 30/01/2023.
//

import SwiftUI

/// Secondary font used in `Polkadot Vault`
enum SecondaryFont {
    /// Black, 28pt
    case titleXL
    /// Black, 24pt
    case titleL
    /// Black, 22pt
    case titleM
    /// Black, 16pt
    case titleS
    /// Bold, 17pt
    case labelL
    /// Bold, 16pt
    case labelM
    /// Bold, 14pt
    case labelS
    /// Regular, 16pt
    case bodyL
    /// Regular, 14pt
    case bodyM
    /// Regular, 12pt
    case captionM
    /// Regular, 10pt
    case captionS
}

extension SecondaryFont {
    var font: SwiftUI.Font {
        switch self {
        case .titleXL:
            FontFamily.Unbounded.black.swiftUIFont(size: 28)
        case .titleL:
            FontFamily.Unbounded.black.swiftUIFont(size: 24)
        case .titleM:
            FontFamily.Unbounded.black.swiftUIFont(size: 22)
        case .titleS:
            FontFamily.Unbounded.black.swiftUIFont(size: 16)
        case .labelL:
            FontFamily.Unbounded.bold.swiftUIFont(size: 17)
        case .labelM:
            FontFamily.Unbounded.bold.swiftUIFont(size: 16)
        case .labelS:
            FontFamily.Unbounded.bold.swiftUIFont(size: 14)
        case .bodyL:
            FontFamily.Unbounded.regular.swiftUIFont(size: 16)
        case .bodyM:
            FontFamily.Unbounded.regular.swiftUIFont(size: 14)
        case .captionM:
            FontFamily.Unbounded.regular.swiftUIFont(size: 12)
        case .captionS:
            FontFamily.Unbounded.regular.swiftUIFont(size: 10)
        }
    }
}
