//
//  KeyDetailsActionsModal.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 06/09/2022.
//

import SwiftUI

struct KeyDetailsActionsModal: View {
    @State private var animateBackground: Bool = false
    @Binding var isShowingActionSheet: Bool
    @Binding var shouldPresentRemoveConfirmationModal: Bool
    @Binding var shouldPresentBackupModal: Bool
    @Binding var shouldPresentExportKeysSelection: Bool

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: { animateDismissal() },
            animateBackground: $animateBackground,
            content: {
                VStack(alignment: .leading, spacing: 0) {
                    // Export Keys
                    ActionSheetButton(
                        action: { animateDismissal { shouldPresentExportKeysSelection.toggle() } },
                        icon: Image(.exportKeys),
                        text: Localizable.KeySetsModal.Action.export.key
                    )
                    ActionSheetButton(
                        action: {
                            animateDismissal { shouldPresentBackupModal.toggle() }
                        },
                        icon: Image(.backupKey),
                        text: Localizable.KeySetsModal.Action.backup.key
                    )
                    ActionSheetButton(
                        action: { animateDismissal { shouldPresentRemoveConfirmationModal.toggle() } },
                        icon: Image(.delete),
                        text: Localizable.KeySetsModal.Action.delete.key,
                        style: .destructive
                    )
                    ActionButton(
                        action: { animateDismissal() },
                        text: Localizable.AddKeySet.Button.cancel.key,
                        style: .emptySecondary()
                    )
                }
                .padding(.horizontal, Spacing.large)
                .padding(.top, -Spacing.extraSmall)
                .padding(.bottom, Spacing.medium)
            }
        )
    }

    private func animateDismissal(_ completion: @escaping () -> Void = {}) {
        Animations.chainAnimation(
            animateBackground.toggle(),
            delayedAnimationClosure: {
                isShowingActionSheet = false
                completion()
            }()
        )
    }
}
