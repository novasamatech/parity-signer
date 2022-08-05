//
//  Fonts.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 8.12.2021.
//

import Foundation
import SwiftUI

//
// Fonts definitions
//
// Use this place only to define fonts!
//

// Fontstyles
enum Fontstyle {
    case h1 // swiftlint:disable:this identifier_name
    case h2 // swiftlint:disable:this identifier_name
    case h3 // swiftlint:disable:this identifier_name
    case h4 // swiftlint:disable:this identifier_name
    case button
    case body1
    case body2
    case overline
    case subtitle1
    case subtitle2
}

// Base
func FBase(style: Fontstyle) -> Font {
    switch style {
    case .h1:
        return Font.custom("Inter-Bold", size: 19)
    case .h2:
        return Font.custom("Inter-Semibold", size: 19)
    case .h3:
        return Font.custom("Inter-Semibold", size: 16)
    case .h4:
        return Font.custom("Inter-Medium", size: 16)
    case .button:
        return Font.custom("Inter-Semibold", size: 17)
    case .body1:
        return Font.custom("Inter-Regular", size: 16)
    case .body2:
        return Font.custom("Inter-Regular", size: 15)
    case .overline:
        return Font.custom("Inter-Medium", size: 13)
    case .subtitle1:
        return Font.custom("Inter-Medium", size: 15)
    case .subtitle2:
        return Font.custom("Inter-Regular", size: 13)
    }
}

// Crypto
func FCrypto(style: Fontstyle) -> Font {
    switch style {
    case .h1:
        return Font.custom("RobotoMono-Medium", size: 19)
    case .h2:
        return Font.custom("RobotoMono-Medium", size: 19)
    case .h3:
        return Font.custom("RobotoMono-Medium", size: 16)
    case .h4:
        return Font.custom("RobotoMono-Medium", size: 16)
    case .button:
        return Font.custom("RobotoMono-Medium", size: 16)
    case .body1:
        return Font.custom("RobotoMono-Medium", size: 12)
    case .body2:
        return Font.custom("RobotoMono-Light", size: 12).weight(.medium)
    case .overline:
        return Font.custom("RobotoMono-Medium", size: 12)
    case .subtitle1:
        return Font.custom("RobotoMono-Medium", size: 14)
    case .subtitle2:
        return Font.custom("RobotoMono-Medium", size: 13)
    }
}

// Web3
func FWeb3(style: Fontstyle) -> Font {
    switch style {
    case .h1:
        return Font.custom("Web3-Regular", size: 19)
    case .h2:
        return Font.custom("Web3-Regular", size: 19)
    case .h3:
        return Font.custom("Web3-Regular", size: 16)
    case .h4:
        return Font.custom("Web3-Regular", size: 16)
    case .button:
        return Font.custom("Web3-Regular", size: 16)
    case .body1:
        return Font.custom("Web3-Regular", size: 16)
    case .body2:
        return Font.custom("Web3-Regular", size: 15)
    case .overline:
        return Font.custom("Web3-Regular", size: 12)
    case .subtitle1:
        return Font.custom("Web3-Regular", size: 14)
    case .subtitle2:
        return Font.custom("Web3-Regular", size: 13)
    }
}
