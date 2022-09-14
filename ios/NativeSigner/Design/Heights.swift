//
//  Heights.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 23/08/2022.
//

import UIKit

/// Common UI components heights to be used for Design System views
enum Heights {
    /// All variants of `ActionButton`, 56 pt
    static let actionButton: CGFloat = 56
    /// All variants of `ActionButton`, 56 pt

    static let snackbarHeight: CGFloat = 56
    /// All variants of `NavbarButton`, 40 pt
    static let navigationButton: CGFloat = 40
    /// All variants of `MenuButton`, 48 pt
    static let menuButton: CGFloat = 48
    /// All variants of `ActionSheetButton`, 44 pt
    static let actionSheetButton: CGFloat = 44
    /// Height for cell container for Key Set collection element
    static let keyCellContainer: CGFloat = 72
    /// Height for `Identicon` when used in list collections
    static let identiconInCell: CGFloat = 36
}

enum Sizes {
    /// Diameter for "X" close button on modals, 32pt
    static let xmarkButtonDiameter: CGFloat = 32
    /// Size for left-aligned icons within `MenuButton` / `ActionSheetButton`, 30pt
    static let actionSheetIcon: CGFloat = 30
}
