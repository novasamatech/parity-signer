//
//  Heights.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 23/08/2022.
//

import UIKit

/// Common UI components heights to be used for Design System views
enum Heights {
    static let chevronRightInList: CGFloat = 14
    static let oviewviewPageIndicator: CGFloat = 4
    /// All variants of `ActionButton`, 56 pt
    static let actionButton: CGFloat = 56
    /// All variants of `Snackbar`, 56 pt
    static let snackbarHeight: CGFloat = 56
    /// Max height variant of `Snackbar`, 72 pt
    static let snackbarMaxHeight: CGFloat = 72
    /// All variants of `NavigationBarView`, 54 pt
    static let navigationBarHeight: CGFloat = 54
    /// All variants of `NavbarButton`, 40 pt
    static let navigationButton: CGFloat = 40
    /// All variants of `IconButton`, 36 pt
    static let iconButton: CGFloat = 36
    /// All variants of `MenuButton`, 48 pt
    static let menuButton: CGFloat = 48
    /// All variants of `ActionSheetButton`, 44 pt
    static let actionSheetButton: CGFloat = 44
    /// Height for cell container for Key Set collection element
    static let keyCellContainer: CGFloat = 72
    /// Height for `Identicon` when used in list collections
    static let identiconInCell: CGFloat = 36
    static let identiconInManageKeySet: CGFloat = 40

    static let identiconRootKeyDetails: CGFloat = 56
    /// Height for `Identicon` when used as inline icon
    static let identiconSmall: CGFloat = 16
    static let identiconInAddDerivedKey: CGFloat = 40
    static let tabbarHeight: CGFloat = 49
    static let textFieldHeight: CGFloat = 48
    static let seedPhraseCapsuleHeight: CGFloat = 32
    static let minTextEditorHeight: CGFloat = 96
    static let maxTextEditorHeight: CGFloat = 230
    static let tabbarAssetHeight: CGFloat = 28
    static let tabbarScannerHeight: CGFloat = 41
    static let errorModalIconContainer: CGFloat = 80
    /// All variants of `ProgressSnackbar`, 96 pt
    static let progressSnackbarHeight: CGFloat = 96
    static let bottomBarHeight: CGFloat = 56
    static let capsuleButton: CGFloat = 40
    static let minTransactionCardHeight: CGFloat = 24
    static let minTransactionSummaryItemHeight: CGFloat = 18
    static let chevronLogElementWidth: CGFloat = 32
    /// Height for `Network Logo` when used in list collections, 36 pt
    static let networkLogoOverlay: CGFloat = 18
    /// Height for `Network Logo` when used in list collections, 36 pt
    static let networkLogoInCell: CGFloat = 36
    static let networkLogoInModal: CGFloat = 80
    static let networkLogoInList: CGFloat = 32
    static let networkLogoInHeader: CGFloat = 56
    /// Height for `Network Logo` when used in small capsule view, 24 pt
    static let networkLogoInCapsule: CGFloat = 24
    /// Height for element in Network Filter modal
    static let networkFilterItem: CGFloat = 48
    static let settingsEntryHeight: CGFloat = 56
    static let settingsSelectKeyEntryHeight: CGFloat = 60

    static let verifierCertificateActionHeight: CGFloat = 48
    /// Height for element in Network Selection Settings
    static let networkSelectionSettings: CGFloat = 52
    static let selectionBox: CGFloat = 48
    static let capsuleSelectionView: CGFloat = 28
    static let minimumActionSheetButtonHeight: CGFloat = 48
    static let onboardingAgreementRecord: CGFloat = 52

    static let signSpecsListRowHeight: CGFloat = 96

    static let createKeyNetworkItemHeight: CGFloat = 64
    static let createKeysForNetworkItemHeight: CGFloat = 64
    static let selectKeySetsForNetworkKeyItemHeight: CGFloat = 64
    static let exportKeysSelectionCellHeight: CGFloat = 64

    static let navigationBarProgressViewHeight: CGFloat = 6
    static let navigationBarProgressViewWidth: CGFloat = 40
    static let manageKeySetSelectionIcon: CGFloat = 32
}

enum Sizes {
    /// Diameter for "X" close button on modals, 32pt
    static let xmarkButtonDiameter: CGFloat = 32
    /// Size for left-aligned icons within `MenuButton` / `ActionSheetButton`, 30pt
    static let actionSheetIcon: CGFloat = 30
    static let actionSheetCircleIcon: CGFloat = 40
    static let checkmarkCircleButton: CGFloat = 32
    /// Size for seed word position label
    static let seedWordPositionWidth: CGFloat = 18
    /// Diameter for ">" button in circle, 28pt
    static let chevronCircleButton: CGFloat = 28
    static let rightChevronContainerSize: CGFloat = 28
    static let roundedQuestionmark: CGFloat = 24
    static let pointCircle: CGFloat = 32
    static let signSpecsIdenticonSize: CGFloat = 36
    static let chevronDownKeyDetails: CGFloat = 16
}
