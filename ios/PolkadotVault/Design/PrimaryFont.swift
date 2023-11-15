//
//  PrimaryFont.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 08/12/2022.
//

import SwiftUI

/// Base font used in `Polkadot Vault`
enum PrimaryFont {
    /// Bold, 28pt
    case titleXL
    /// Bold, 24pt
    case titleL
    /// Bold, 22pt
    case titleM
    /// Bold, 16pt
    case titleS
    /// Semibold, 17pt
    case labelL
    /// Semibold, 16pt
    case labelM
    /// Semibold, 14pt
    case labelS
    /// Medium, 13pt
    case labelXS
    /// Semibold, 12pt
    case labelXXS
    /// Regular, 16pt
    case bodyL
    /// Regular, 14pt
    case bodyM
    /// Regular, 12pt
    case captionM
    /// Regular, 10pt
    case captionS
}

extension PrimaryFont {
    var font: SwiftUI.Font {
        switch self {
        case .titleXL:
            FontFamily.Inter.bold.swiftUIFont(size: 28)
        case .titleL:
            FontFamily.Inter.bold.swiftUIFont(size: 24)
        case .titleM:
            FontFamily.Inter.bold.swiftUIFont(size: 22)
        case .titleS:
            FontFamily.Inter.bold.swiftUIFont(size: 16)
        case .labelL:
            FontFamily.Inter.semiBold.swiftUIFont(size: 17)
        case .labelM:
            FontFamily.Inter.semiBold.swiftUIFont(size: 16)
        case .labelS:
            FontFamily.Inter.semiBold.swiftUIFont(size: 14)
        case .labelXS:
            FontFamily.Inter.medium.swiftUIFont(size: 13)
        case .labelXXS:
            FontFamily.Inter.semiBold.swiftUIFont(size: 12)
        case .bodyL:
            FontFamily.Inter.regular.swiftUIFont(size: 16)
        case .bodyM:
            FontFamily.Inter.regular.swiftUIFont(size: 14)
        case .captionM:
            FontFamily.Inter.regular.swiftUIFont(size: 12)
        case .captionS:
            FontFamily.Inter.regular.swiftUIFont(size: 10)
        }
    }
}
