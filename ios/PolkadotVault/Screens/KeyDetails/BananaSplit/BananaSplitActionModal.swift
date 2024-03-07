//
//  BananaSplitActionModal.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 23/02/2024.
//

import SwiftUI

struct BananaSplitActionModal: View {
    @StateObject var viewModel: ViewModel

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: { viewModel.dismissActionSheet() },
            animateBackground: $viewModel.animateBackground,
            content: {
                VStack(alignment: .leading, spacing: 0) {
                    // Show Passphrase Keys
                    ActionSheetButton(
                        action: viewModel.showPassphrase,
                        icon: Image(.showPassphrase),
                        text: Localizable.BananaSplitActionModal.Action.passphrase.key
                    )
                    // Remove Keys
                    ActionSheetButton(
                        action: viewModel.removeBackup,
                        icon: Image(.delete),
                        text: Localizable.BananaSplitActionModal.Action.remove.key,
                        style: .destructive
                    )
                    // Cancel
                    ActionButton(
                        action: viewModel.dismissActionSheet,
                        text: Localizable.BananaSplitActionModal.Action.cancel.key,
                        style: .emptySecondary()
                    )
                }
                .padding(.horizontal, Spacing.large)
                .padding(.top, -Spacing.extraSmall)
                .padding(.bottom, Spacing.medium)
            }
        )
    }
}

extension BananaSplitActionModal {
    final class ViewModel: ObservableObject {
        @Published var animateBackground: Bool = false
        @Binding var isPresented: Bool
        @Binding var shouldPresentDeleteBackupWarningModal: Bool
        @Binding var shouldPresentPassphraseModal: Bool

        init(
            isPresented: Binding<Bool>,
            shouldPresentDeleteBackupWarningModal: Binding<Bool>,
            shouldPresentPassphraseModal: Binding<Bool>
        ) {
            _isPresented = isPresented
            _shouldPresentDeleteBackupWarningModal = shouldPresentDeleteBackupWarningModal
            _shouldPresentPassphraseModal = shouldPresentPassphraseModal
        }

        func removeBackup() {
            shouldPresentDeleteBackupWarningModal = true
            dismissActionSheet()
        }

        func showPassphrase() {
            shouldPresentPassphraseModal = true
            dismissActionSheet()
        }

        func dismissActionSheet() {
            animateDismissal()
        }

        func animateDismissal() {
            Animations.chainAnimation(
                animateBackground.toggle(),
                // swiftformat:disable all
                delayedAnimationClosure: self.hide()
            )
        }
        private func hide() {
            isPresented = false
        }
    }
}
