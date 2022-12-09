//
//  PrimaryFont.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 08/12/2022.
//

import SwiftUI

/// Base font used in `Signer`
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
            return FontFamily.Inter.bold.swiftUIFont(size: 28)
        case .titleL:
            return FontFamily.Inter.bold.swiftUIFont(size: 24)
        case .titleM:
            return FontFamily.Inter.bold.swiftUIFont(size: 22)
        case .titleS:
            return FontFamily.Inter.bold.swiftUIFont(size: 16)
        case .labelL:
            return FontFamily.Inter.semiBold.swiftUIFont(size: 17)
        case .labelM:
            return FontFamily.Inter.semiBold.swiftUIFont(size: 16)
        case .labelS:
            return FontFamily.Inter.semiBold.swiftUIFont(size: 14)
        case .bodyL:
            return FontFamily.Inter.regular.swiftUIFont(size: 16)
        case .bodyM:
            return FontFamily.Inter.regular.swiftUIFont(size: 14)
        case .captionM:
            return FontFamily.Inter.regular.swiftUIFont(size: 12)
        case .captionS:
            return FontFamily.Inter.regular.swiftUIFont(size: 10)
        }
    }
}
