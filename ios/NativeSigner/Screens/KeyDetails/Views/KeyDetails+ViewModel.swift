//
//  KeyDetails+ViewModel.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 27/10/2022.
//

import Foundation

extension KeyDetailsView {
    final class ViewModel: ObservableObject {
        let keyDetailsService: KeyDetailsService
        let exportPrivateKeyService: PrivateKeyQRCodeService

        /// `MKwysNew` will currently be `nil` when navigating through given navigation path:
        /// `.newSeed` -> `.keys`, data will be filled on `onAppear`, so this can remain optional
        var keysData: MKeysNew?
        private weak var appState: AppState!
        private weak var navigation: NavigationCoordinator!
        @Published var shouldPresentRemoveConfirmationModal = false
        @Published var shouldPresentBackupModal = false
        @Published var shouldPresentSelectionOverlay = false
        @Published var isShowingActionSheet = false
        @Published var isShowingRemoveConfirmation = false
        @Published var isShowingBackupModal = false
        @Published var isPresentingConnectivityAlert = false
        @Published var isPresentingSelectionOverlay = false
        @Published var isShowingKeysExportModal = false
        @Published var selectedSeeds: [String] = []
        // Network selection
        @Published var isPresentingNetworkSelection = false

        init(
            keysData: MKeysNew?,
            exportPrivateKeyService: PrivateKeyQRCodeService,
            keyDetailsService: KeyDetailsService = KeyDetailsService()
        ) {
            self.keysData = keysData
            self.exportPrivateKeyService = exportPrivateKeyService
            self.keyDetailsService = keyDetailsService
        }

        func use(appState: AppState) {
            self.appState = appState
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func keyExportModel(dataModel: KeyDetailsDataModel) -> ExportMultipleKeysModalViewModel {
            let derivedKeys: [DerivedKeyExportModel] = dataModel.derivedKeys
                .filter { selectedSeeds.contains($0.viewModel.path) }
                .compactMap {
                    guard let keyData = keyData(for: $0.viewModel.path) else { return nil }
                    return DerivedKeyExportModel(viewModel: $0.viewModel, keyData: keyData)
                }
            return ExportMultipleKeysModalViewModel(
                selectedItems: .keys(
                    key: dataModel.keySummary,
                    derivedKeys: derivedKeys
                ),
                seedNames: selectedSeeds
            )
        }

        func refreshData(dataModel: KeyDetailsDataModel) {
            keyDetailsService.getKeys(for: dataModel.keySummary.keyName) { result in
                if case let .success(keysData) = result {
                    self.appState.userData.keysData = keysData
                    self.keysData = keysData
                }
            }
        }

        func onActionSheetDismissal(_ isAlertVisible: Bool) {
            if shouldPresentRemoveConfirmationModal {
                shouldPresentRemoveConfirmationModal.toggle()
                isShowingRemoveConfirmation.toggle()
            }
            if shouldPresentBackupModal {
                shouldPresentBackupModal.toggle()
                if isAlertVisible {
                    DispatchQueue.main.asyncAfter(deadline: .now() + 0.1) {
                        self.isPresentingConnectivityAlert.toggle()
                    }
                } else {
                    KeyDetailsView.backupModalViewModel = exportPrivateKeyService.backupViewModel()
                    isShowingBackupModal.toggle()
                }
            }
            if shouldPresentSelectionOverlay {
                shouldPresentSelectionOverlay.toggle()
                isPresentingSelectionOverlay.toggle()
            }
        }

        func onNetworkSelectionTap() {
            guard case let .networkSelector(networksContainer) = navigation
                .performFake(navigation: .init(action: .networkSelector)).modalData else { return }

            appState.userData.allNetworks = networksContainer.networks
            appState.userData.selectedNetworks = networksContainer.networks.filter(\.selected)
            isPresentingNetworkSelection = true
        }
    }
}

private extension KeyDetailsView.ViewModel {
    func keyData(for path: String) -> MKeyAndNetworkCard? {
        keysData?.set.first(where: { $0.key.address.path == path })
    }
}
