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
        }
    }
}
