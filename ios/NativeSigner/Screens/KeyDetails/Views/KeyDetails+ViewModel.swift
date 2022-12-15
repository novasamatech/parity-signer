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
        let keyName: String
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
        // Network selection
        @Published var isPresentingNetworkSelection = false

        @Published var keySummary: KeySummaryViewModel?
        @Published var derivedKeys: [DerivedKeyRowModel] = []
        @Published var selectedSeeds: [String] = []
        private var cancelBag = CancelBag()

        /// Navigation action for selecting main `Address Key`
        var addressKeyNavigation: Navigation?
        /// Collection of navigation actions for tapping on `Derived Key`
        var derivedKeysNavigation: [Navigation] = []
        /// Navigation for `Create Derived Key`
        let createDerivedKey: Navigation = .init(action: .newKey)
        /// Name of seed to be removed with `Remove Seed` action
        var removeSeed: String = ""

        init(
            keyName: String,
            exportPrivateKeyService: PrivateKeyQRCodeService,
            keyDetailsService: KeyDetailsService = KeyDetailsService()
        ) {
            self.keyName = keyName
            self.exportPrivateKeyService = exportPrivateKeyService
            self.keyDetailsService = keyDetailsService
            updateRenderables()
        }

        func use(appState: AppState) {
            self.appState = appState
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func updateRenderables() {
            refreshDerivedKeys()
            refreshKeySummary()
        }

        func refreshData() {
            keyDetailsService.getKeys(for: keyName) { result in
                if case let .success(keysData) = result {
                    self.appState.userData.keysData = keysData
                    self.keysData = keysData
                    self.updateRenderables()
                }
            }
        }
    }
}

// MARK: - Tap Actions

extension KeyDetailsView.ViewModel {
    func onRootKeyTap() {
        guard let addressKeyNavigation = addressKeyNavigation, !isPresentingSelectionOverlay else { return }
        navigation.perform(navigation: addressKeyNavigation)
    }

    func onNetworkSelectionTap() {
        guard case let .networkSelector(networksContainer) = navigation
            .performFake(navigation: .init(action: .networkSelector)).modalData else { return }

        appState.userData.allNetworks = networksContainer.networks
        isPresentingNetworkSelection = true
    }

    func onDerivedKeyTap(_ deriveKey: DerivedKeyRowModel) {
        if isPresentingSelectionOverlay {
            let seedName = deriveKey.viewModel.path
            if selectedSeeds.contains(seedName) {
                selectedSeeds.removeAll { $0 == seedName }
            } else {
                selectedSeeds.append(seedName)
            }
        } else {
            navigation.perform(navigation: deriveKey.actionModel.tapAction)
        }
    }
}

// MARK: - Modals

extension KeyDetailsView.ViewModel {
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

    func keyExportModel() -> ExportMultipleKeysModalViewModel? {
        guard let keySummary = keySummary else { return nil }
        let derivedKeys: [DerivedKeyExportModel] = derivedKeys
            .filter { selectedSeeds.contains($0.viewModel.path) }
            .compactMap {
                guard let keyData = keyData(for: $0.viewModel.path) else { return nil }
                return DerivedKeyExportModel(viewModel: $0.viewModel, keyData: keyData)
            }
        return ExportMultipleKeysModalViewModel(
            selectedItems: .keys(
                key: keySummary,
                derivedKeys: derivedKeys
            ),
            seedNames: selectedSeeds
        )
    }
}

private extension KeyDetailsView.ViewModel {
    func keyData(for path: String) -> MKeyAndNetworkCard? {
        keysData?.set.first(where: { $0.key.address.path == path })
    }

    func refreshDerivedKeys() {
        guard let keysData = keysData else { return }
        let sortedDerivedKeys = keysData.set
            .sorted(by: { $0.key.address.path < $1.key.address.path })
        derivedKeys = sortedDerivedKeys
            .map {
                let details = "\($0.key.addressKey)\n\($0.network.networkSpecsKey)"
                return DerivedKeyRowModel(
                    viewModel: DerivedKeyRowViewModel($0.key),
                    actionModel: DerivedKeyActionModel(
                        tapAction: .init(action: .selectKey, details: details)
                    )
                )
            }
        derivedKeysNavigation = sortedDerivedKeys
            .map {
                let details = "\($0.key.addressKey)\n\($0.network.networkSpecsKey)"
                return .init(action: .selectKey, details: details)
            }
    }

    func refreshKeySummary() {
        guard let keysData = keysData else { return }
        keySummary = KeySummaryViewModel(
            keyName: keysData.root?.address.seedName ?? "",
            base58: keysData.root?.base58 ?? ""
        )
        let details = "\(keysData.root?.addressKey ?? "")\n\(keysData.set.first?.network.networkSpecsKey ?? "")"
        addressKeyNavigation = .init(action: .selectKey, details: details)
        removeSeed = keysData.root?.address.seedName ?? ""
    }
}
