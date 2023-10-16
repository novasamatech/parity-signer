//
//  Spacing.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 19/08/2022.
//

import SwiftUI
import UIKit

enum Spacing {
    /// No padding: 0 pts
    static let none: CGFloat = 0
    /// Stroke: 0.5 pts
    static let stroke: CGFloat = 0.5
    /// Minimal: 2 pts
    static let minimal: CGFloat = 2
    /// Extra Extra Small: 4 pts
    static let extraExtraSmall: CGFloat = 4
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
    /// Extra Extra Extra Large: 48 pts
    static let x3Large: CGFloat = 48
    /// Spacing for dedicated spacer: 60 pts
    static let componentSpacer: CGFloat = 60
    /// Spacing for dedicated spacer: 70 pts
    static let backupComponentSpacer: CGFloat = 70
    /// Large spacing for dedicated spacer: 90 pts
    static let largeComponentSpacer: CGFloat = 90
    /// Spacing for top area offset: 20 pts
    static let topSafeAreaSpacing: CGFloat = 20
    static let screenshotIconCompensation: CGFloat = 20
    /// Spacing for dedicated spacer: 40 / 60 pts
    static var flexibleComponentSpacer: CGFloat {
        switch UIScreen.main.bounds.width {
        case DeviceConstants.compactDeviceWidth:
            40
        default:
            60
        }
    }

    /// Spacing for dedicated spacer: 30 / 60 pts
    static var flexibleSmallComponentSpacer: CGFloat {
        switch UIScreen.main.bounds.width {
        case DeviceConstants.compactDeviceWidth:
            30
        default:
            60
        }
    }
}
