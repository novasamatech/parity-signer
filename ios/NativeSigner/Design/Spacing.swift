//
//  Padding.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 19/08/2022.
//

import SwiftUI
import UIKit

enum Spacing {
    /// No padding: 0 pts
    static let none: CGFloat = 0
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
    /// Spacing for dedicated spacer: 60 pts
    static let componentSpacer: CGFloat = 60
}

enum Padding {
    /// To be used on clickable cells and headers (top: 8, leading: 24, bottom: 8, trailing: 24)
    static let detailsCell: EdgeInsets = .init(
        top: Spacing.extraSmall,
        leading: Spacing.large,
        bottom: Spacing.extraSmall,
        trailing: Spacing.large
    )
}
