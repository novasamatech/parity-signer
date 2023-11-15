//
//  KeyDetailsView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 29/08/2022.
//

import SwiftUI

struct KeyDetailsView: View {
    @StateObject var viewModel: ViewModel

    var body: some View {
        NavigationView {
            ZStack(alignment: .bottom) {
                VStack(spacing: 0) {
                    // Navigation bar
                    NavigationBarView(
                        viewModel: .init(
                            leftButtons: [
                                .init(type: .settings, action: viewModel.onSettingsTap)
                            ],
                            rightButtons: [
                                .init(type: .plus, action: viewModel.onCreateDerivedKeyTap),
                                .init(type: .more, action: viewModel.onMoreTap)
                            ]
                        )
                    )
                    switch viewModel.viewState {
                    case .list:
                        derivedKeysList()
                    case .emptyState:
                        rootKeyHeader()
                        Spacer()
                        emptyState()
                        Spacer()
                    }
                }
                .navigationBarHidden(true)
                .navigationViewStyle(.stack)
                .onAppear {
                    viewModel.onAppear()
                }
                .background(.backgroundPrimary)
                HStack(alignment: .center) {
                    Spacer()
                    QRCodeButton(action: viewModel.onQRScannerTap)
                        .padding(.bottom, Spacing.extraLarge)
                    Spacer()
                }
            }
        }
        .navigationViewStyle(.stack)
        .fullScreenModal(
            isPresented: $viewModel.isPresentingQRScanner,
            onDismiss: viewModel.onQRScannerDismiss
        ) {
            CameraView(
                viewModel: .init(
                    isPresented: $viewModel.isPresentingQRScanner
                )
            )
        }
        .fullScreenModal(
            isPresented: $viewModel.isShowingActionSheet,
            onDismiss: {
                // iOS 15 handling of following .fullscreen presentation after dismissal, we need to dispatch this async
                DispatchQueue.main.async { viewModel.onActionSheetDismissal() }
            }
        ) {
            KeyDetailsActionsModal(
                isShowingActionSheet: $viewModel.isShowingActionSheet,
                shouldPresentRemoveConfirmationModal: $viewModel.shouldPresentRemoveConfirmationModal,
                shouldPresentBackupModal: $viewModel.shouldPresentBackupModal,
                shouldPresentExportKeysSelection: $viewModel.shouldPresentExportKeysSelection
            )
            .clearModalBackground()
        }
        .fullScreenModal(
            isPresented: $viewModel.isPresentingExportKeySelection
        ) {
            ExportKeysSelectionModal(
                viewModel: .init(
                    rootKey: viewModel.keysData?.root?.base58 ?? "",
                    rootIdenticon: viewModel.keysData?.root?.address.identicon,
                    derivedKeys: viewModel.derivedKeys,
                    isPresented: $viewModel.isPresentingExportKeySelection,
                    onCompletion: viewModel.onExportKeySelectionComplete
                )
            )
            .clearModalBackground()
        }
        .fullScreenModal(
            isPresented: $viewModel.isPresentingKeySetSelection
        ) {
            ManageKeySetsView(
                viewModel: .init(
                    isPresented: $viewModel.isPresentingKeySetSelection,
                    currentKeySet: viewModel.keyName,
                    onCompletion: viewModel.onKeySetSelectionComplete
                )
            )
            .clearModalBackground()
        }
        .fullScreenModal(isPresented: $viewModel.isShowingRemoveConfirmation) {
            HorizontalActionsBottomModal(
                viewModel: .forgetKeySet,
                mainAction: viewModel.onRemoveKeySetConfirmationTap(),
                isShowingBottomAlert: $viewModel.isShowingRemoveConfirmation
            )
            .clearModalBackground()
        }
        .fullScreenModal(
            isPresented: $viewModel.isShowingBackupModal,
            onDismiss: viewModel.clearBackupModalState
        ) {
            if let viewModel = viewModel.backupModal {
                BackupModal(
                    isShowingBackupModal: $viewModel.isShowingBackupModal,
                    viewModel: viewModel
                )
                .clearModalBackground()
            } else {
                EmptyView()
            }
        }
        .fullScreenModal(
            isPresented: $viewModel.isShowingKeysExportModal
        ) {
            if let keyExportModel = viewModel.keysExportModalViewModel?() {
                ExportMultipleKeysModal(
                    viewModel: .init(
                        viewModel: keyExportModel,
                        isPresented: $viewModel.isShowingKeysExportModal
                    )
                )
                .clearModalBackground()
            } else {
                EmptyView()
            }
        }
        .fullScreenModal(
            isPresented: $viewModel.isPresentingNetworkSelection
        ) {
            NetworkSelectionModal(
                viewModel: .init(
                    isPresented: $viewModel.isPresentingNetworkSelection,
                    networks: $viewModel.networks,
                    selectedNetworks: $viewModel.selectedNetworks
                )
            )
            .clearModalBackground()
        }
        .fullScreenModal(
            isPresented: $viewModel.isPresentingRootDetails
        ) {
            RootKeyDetailsModal(
                viewModel: .init(
                    renderable: viewModel.rootKeyDetails(),
                    isPresented: $viewModel.isPresentingRootDetails
                )
            )
            .clearModalBackground()
        }
        .fullScreenModal(
            isPresented: $viewModel.isPresentingError
        ) {
            ErrorBottomModal(
                viewModel: viewModel.presentableError,
                isShowingBottomAlert: $viewModel.isPresentingError
            )
            .clearModalBackground()
        }
        .fullScreenModal(
            isPresented: $viewModel.isShowingCreateKeySet
        ) {
            EnterKeySetNameView(
                viewModel: .init(
                    isPresented: $viewModel.isShowingCreateKeySet,
                    onCompletion: viewModel.onKeySetAddCompletion(_:)
                )
            )
        }
        .fullScreenModal(
            isPresented: $viewModel.isShowingRecoverKeySet
        ) {
            RecoverKeySetNameView(
                viewModel: .init(
                    isPresented: $viewModel.isShowingRecoverKeySet,
                    onCompletion: viewModel.onKeySetAddCompletion(_:)
                )
            )
        }
        .fullScreenModal(
            isPresented: $viewModel.isPresentingDeriveNewKey,
            onDismiss: viewModel.refreshData
        ) {
            NavigationView {
                CreateKeyNetworkSelectionView(viewModel: viewModel.createDerivedKeyViewModel())
                    .navigationViewStyle(.stack)
                    .navigationBarHidden(true)
            }
        }
        .fullScreenModal(
            isPresented: $viewModel.isPresentingSettings
        ) {
            NavigationView {
                SettingsView(viewModel: .init())
                    .navigationViewStyle(.stack)
                    .navigationBarHidden(true)
            }
        }
        .bottomSnackbar(
            viewModel.snackbarViewModel,
            isPresented: $viewModel.isSnackbarPresented
        )
    }
}
