//
//  Fontstyle.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 8.12.2021.
//

import SwiftUI

enum Fontstyle {
    case header1
    case header2
    case header3
    case header4
    case button
    case body1
    case body2
    case overline
    case subtitle1
    case subtitle2

    // New font styles; one above should be deleted after redesign is finished
    case titleXL
    case titleL
    case titleM
    case titleS
    case labelL
    case labelM
    case labelS
    case bodyL
    case bodyM
    case captionM
    case captionS
}

extension Fontstyle {
    var base: SwiftUI.Font {
        switch self {
        case .header1:
            return FontFamily.Inter.bold.swiftUIFont(size: 19)
        case .header2:
            return FontFamily.Inter.semiBold.swiftUIFont(size: 19)
        case .header3:
            return FontFamily.Inter.semiBold.swiftUIFont(size: 16)
        case .header4:
            return FontFamily.Inter.medium.swiftUIFont(size: 16)
        case .button:
            return FontFamily.Inter.semiBold.swiftUIFont(size: 17)
        case .body1:
            return FontFamily.Inter.regular.swiftUIFont(size: 16)
        case .body2:
            return FontFamily.Inter.regular.swiftUIFont(size: 15)
        case .overline:
            return FontFamily.Inter.medium.swiftUIFont(size: 13)
        case .subtitle1:
            return FontFamily.Inter.medium.swiftUIFont(size: 15)
        case .subtitle2:
            return FontFamily.Inter.regular.swiftUIFont(size: 13)
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

extension Fontstyle {
    var crypto: SwiftUI.Font {
        switch self {
        case .header1:
            return FontFamily.RobotoMono.medium.swiftUIFont(size: 19)
        case .header2:
            return FontFamily.RobotoMono.medium.swiftUIFont(size: 19)
        case .header3:
            return FontFamily.RobotoMono.medium.swiftUIFont(size: 16)
        case .header4:
            return FontFamily.RobotoMono.medium.swiftUIFont(size: 16)
        case .button:
            return FontFamily.RobotoMono.medium.swiftUIFont(size: 16)
        case .body1:
            return FontFamily.RobotoMono.medium.swiftUIFont(size: 12)
        case .body2:
            return FontFamily.RobotoMono.light.swiftUIFont(size: 12).weight(.medium)
        case .overline:
            return FontFamily.RobotoMono.medium.swiftUIFont(size: 12)
        case .subtitle1:
            return FontFamily.RobotoMono.medium.swiftUIFont(size: 14)
        case .subtitle2:
            return FontFamily.RobotoMono.medium.swiftUIFont(size: 13)
        default:
            fatalError("Font undefined")
        }
    }
}

extension Fontstyle {
    var web3: SwiftUI.Font {
        switch self {
        case .header1:
            return FontFamily.Web3.regular.swiftUIFont(size: 19)
        case .header2:
            return FontFamily.Web3.regular.swiftUIFont(size: 19)
        case .header3:
            return FontFamily.Web3.regular.swiftUIFont(size: 16)
        case .header4:
            return FontFamily.Web3.regular.swiftUIFont(size: 16)
        case .button:
            return FontFamily.Web3.regular.swiftUIFont(size: 16)
        case .body1:
            return FontFamily.Web3.regular.swiftUIFont(size: 16)
        case .body2:
            return FontFamily.Web3.regular.swiftUIFont(size: 15)
        case .overline:
            return FontFamily.Web3.regular.swiftUIFont(size: 12)
        case .subtitle1:
            return FontFamily.Web3.regular.swiftUIFont(size: 14)
        case .subtitle2:
            return FontFamily.Web3.regular.swiftUIFont(size: 13)
        default:
            fatalError("Font undefined")
        }
    }
}
