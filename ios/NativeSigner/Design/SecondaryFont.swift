//
//  SecondaryFont.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 30/01/2023.
//

import SwiftUI

/// Secondary font used in `Signer`
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
            return FontFamily.Unbounded.black.swiftUIFont(size: 28)
        case .titleL:
            return FontFamily.Unbounded.black.swiftUIFont(size: 24)
        case .titleM:
            return FontFamily.Unbounded.black.swiftUIFont(size: 22)
        case .titleS:
            return FontFamily.Unbounded.black.swiftUIFont(size: 16)
        case .labelL:
            return FontFamily.Unbounded.bold.swiftUIFont(size: 17)
        case .labelM:
            return FontFamily.Unbounded.bold.swiftUIFont(size: 16)
        case .labelS:
            return FontFamily.Unbounded.bold.swiftUIFont(size: 14)
        case .bodyL:
            return FontFamily.Unbounded.regular.swiftUIFont(size: 16)
        case .bodyM:
            return FontFamily.Unbounded.regular.swiftUIFont(size: 14)
        case .captionM:
            return FontFamily.Unbounded.regular.swiftUIFont(size: 12)
        case .captionS:
            return FontFamily.Unbounded.regular.swiftUIFont(size: 10)
        }
    }
}
