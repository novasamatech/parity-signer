//
//  CornerRadius.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 22/08/2022.
//

import UIKit

enum CornerRadius {
    /// No padding: 0 pts
    static let none: CGFloat = 0
    /// Extra Small: 8 pts
    static let extraSmall: CGFloat = 8
    /// Small: 12 pts
    static let small: CGFloat = 12
    /// Medium: 16 pts
    static let medium: CGFloat = 16
    /// Large: 24 pts
    static let large: CGFloat = 24
    /// Extra Large: 32 pts
    static let extraLarge: CGFloat = 32
    /// Extra Extra Large: 40 pts
    static let extraExtraLarge: CGFloat = 40
    /// QR Code scanner: 56pts or 74pts, depending on device width
    static var qrCodeScanner: CGFloat {
        switch UIScreen.main.bounds.width {
        case DeviceConstants.compactDeviceWidth:
            56
        default:
            74
        }
    }
}

enum DeviceConstants {
    static let compactDeviceWidth: CGFloat = 320
}
