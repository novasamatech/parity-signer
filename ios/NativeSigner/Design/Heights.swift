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
    /// All variants of `Snackbar`, 56 pt
    static let snackbarHeight: CGFloat = 56
    /// All variants of `NavigationBarView`, 64 pt
    static let navigationBarHeight: CGFloat = 64
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
    static let tabbarHeight: CGFloat = 49
    static let textFieldHeight: CGFloat = 48
    static let tabbarAssetHeight: CGFloat = 28
    static let errorModalIconContainer: CGFloat = 80
    /// All variants of `ProgressSnackbar`, 96 pt
    static let progressSnackbarHeight: CGFloat = 96
    static let bottomBarHeight: CGFloat = 56
    static let capsuleButton: CGFloat = 40
    static let minTransactionCardHeight: CGFloat = 24
    static let minTransactionSummaryItemHeight: CGFloat = 18
}

enum Sizes {
    /// Diameter for "X" close button on modals, 32pt
    static let xmarkButtonDiameter: CGFloat = 32
    /// Size for left-aligned icons within `MenuButton` / `ActionSheetButton`, 30pt
    static let actionSheetIcon: CGFloat = 30
    /// Size for seed word position label
    static let seedWordPositionWidth: CGFloat = 28
}
