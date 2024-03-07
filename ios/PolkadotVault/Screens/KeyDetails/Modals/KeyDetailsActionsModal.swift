//
//  KeyDetailsActionsModal.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 06/09/2022.
//

import SwiftUI

struct KeyDetailsActionsModal: View {
    @StateObject var viewModel: ViewModel

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: { viewModel.dismissActionSheet() },
            animateBackground: $viewModel.animateBackground,
            content: {
                VStack(alignment: .leading, spacing: 0) {
                    // Export Keys
                    ActionSheetButton(
                        action: viewModel.exportKeysAction,
                        icon: Image(.exportKeys),
                        text: Localizable.KeySetsModal.Action.export.key
                    )
                    // Banana Split Backup
                    ActionSheetButton(
                        action: viewModel.bananaSplitBackup,
                        icon: Image(.bananaSplitBackup),
                        text: Localizable.KeySetsModal.Action.bananaSplit.key
                    )
                    // Manual Backup
                    ActionSheetButton(
                        action: viewModel.manualBackupKeysAction,
                        icon: Image(.backupKey),
                        text: Localizable.KeySetsModal.Action.backup.key
                    )
                    // Remove Keys
                    ActionSheetButton(
                        action: viewModel.removeKeysAction,
                        icon: Image(.delete),
                        text: Localizable.KeySetsModal.Action.delete.key,
                        style: .destructive
                    )
                    // Cancel
                    ActionButton(
                        action: viewModel.dismissActionSheet,
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
}

extension KeyDetailsActionsModal {
    final class ViewModel: ObservableObject {
        @Published var animateBackground: Bool = false
        @Binding var isPresented: Bool
        @Binding var shouldPresentRemoveConfirmationModal: Bool
        @Binding var shouldPresentManualBackupModal: Bool
        @Binding var shouldPresentBananaSplitModal: Bool
        @Binding var shouldPresentExportKeysSelection: Bool

        init(
            isPresented: Binding<Bool>,
            shouldPresentRemoveConfirmationModal: Binding<Bool>,
            shouldPresentBananaSplitModal: Binding<Bool>,
            shouldPresentManualBackupModal: Binding<Bool>,
            shouldPresentExportKeysSelection: Binding<Bool>
        ) {
            _isPresented = isPresented
            _shouldPresentRemoveConfirmationModal = shouldPresentRemoveConfirmationModal
            _shouldPresentBananaSplitModal = shouldPresentBananaSplitModal
            _shouldPresentManualBackupModal = shouldPresentManualBackupModal
            _shouldPresentExportKeysSelection = shouldPresentExportKeysSelection
        }

        func exportKeysAction() {
            shouldPresentExportKeysSelection = true
            dismissActionSheet()
        }

        func bananaSplitBackup() {
            shouldPresentBananaSplitModal = true
            dismissActionSheet()
        }

        func manualBackupKeysAction() {
            shouldPresentManualBackupModal = true
            dismissActionSheet()
        }

        func removeKeysAction() {
            shouldPresentRemoveConfirmationModal = true
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
