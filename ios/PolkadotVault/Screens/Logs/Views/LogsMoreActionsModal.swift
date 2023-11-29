//
//  LogsMoreActionsModal.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 07/12/2022.
//

import SwiftUI

struct LogsMoreActionsModal: View {
    @State private var animateBackground: Bool = false
    @Binding var isShowingActionSheet: Bool
    @Binding var shouldPresentClearConfirmationModal: Bool
    @Binding var shouldPresentAddNoteModal: Bool

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: { animateDismissal() },
            animateBackground: $animateBackground,
            content: {
                VStack(alignment: .leading, spacing: 0) {
                    // Add Note
                    ActionSheetButton(
                        action: { animateDismissal { shouldPresentAddNoteModal.toggle() } },
                        icon: Image(.addLarge),
                        text: Localizable.LogsList.More.Action.add.key
                    )
                    // Clear Log
                    ActionSheetButton(
                        action: { animateDismissal { shouldPresentClearConfirmationModal.toggle() } },
                        icon: Image(.delete),
                        text: Localizable.LogsList.More.Action.clear.key,
                        style: .destructive
                    )
                    ActionButton(
                        action: { animateDismissal() },
                        text: Localizable.LogsList.More.Action.cancel.key,
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
