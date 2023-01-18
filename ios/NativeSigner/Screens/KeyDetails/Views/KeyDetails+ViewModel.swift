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
        private let networksService: GetAllNetworksService
        private let cancelBag = CancelBag()
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
        @Published var isPresentingRootDetails = false
        @Published var isShowingKeysExportModal = false
        // Network selection
        @Published var isPresentingNetworkSelection = false

        @Published var keySummary: KeySummaryViewModel?
        @Published var derivedKeys: [DerivedKeyRowModel] = []
        @Published var selectedSeeds: [String] = []
        @Published var isFilteringActive: Bool = false
        // Error handling
        @Published var isPresentingError: Bool = false
        @Published var presentableError: ErrorBottomModalViewModel = .noNetworksAvailable()

        /// Name of seed to be removed with `Remove Seed` action
        var removeSeed: String = ""

        init(
            keyName: String,
            exportPrivateKeyService: PrivateKeyQRCodeService = PrivateKeyQRCodeService(),
            keyDetailsService: KeyDetailsService = KeyDetailsService(),
            networksService: GetAllNetworksService = GetAllNetworksService()
        ) {
            self.keyName = keyName
            self.exportPrivateKeyService = exportPrivateKeyService
            self.keyDetailsService = keyDetailsService
            self.networksService = networksService
            updateRenderables()
            subscribeToNetworkChanges()
        }

        func use(appState: AppState) {
            self.appState = appState
            $isPresentingNetworkSelection.sink { newValue in
                guard !newValue else { return }
                self.isFilteringActive = !self.appState.userData.selectedNetworks.isEmpty
            }
            .store(in: cancelBag)
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func subscribeToNetworkChanges() {
            $isPresentingNetworkSelection.sink { newValue in
                guard !newValue else { return }
                self.refreshDerivedKeys()
            }
            .store(in: cancelBag)
        }

        func updateRenderables() {
            refreshDerivedKeys()
            refreshKeySummary()
            refreshNetworks()
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

        func refreshNetworks() {
            networksService.getNetworks { result in
                if case let .success(networks) = result {
                    self.appState.userData.allNetworks = networks
                }
            }
        }
    }
}

// MARK: - Tap Actions

extension KeyDetailsView.ViewModel {
    func onCreateDerivedKeyTap() {
        if !appState.userData.allNetworks.isEmpty {
            navigation.perform(navigation: .init(action: .newKey))
        } else {
            presentableError = .noNetworksAvailable()
            isPresentingError = true
        }
    }

    func onRootKeyTap() {
        guard !isPresentingSelectionOverlay else { return }
        isPresentingRootDetails = true
    }

    func onNetworkSelectionTap() {
        networksService.getNetworks { result in
            if case let .success(networks) = result {
                self.appState.userData.allNetworks = networks
                self.isPresentingNetworkSelection = true
            }
        }
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
                isShowingBackupModal.toggle()
            }
        }
        if shouldPresentSelectionOverlay {
            shouldPresentSelectionOverlay.toggle()
            isPresentingSelectionOverlay.toggle()
        }
    }

    func backupViewModel() -> BackupModalViewModel? {
        exportPrivateKeyService.backupViewModel(keysData)
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

    func rootKeyDetails() -> RootKeyDetailsModal.ViewModel {
        .init(name: keySummary?.keyName ?? "", publicKey: keySummary?.base58 ?? "")
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
        let filteredKeys: [MKeyAndNetworkCard]
        if appState.userData.selectedNetworks.isEmpty {
            filteredKeys = sortedDerivedKeys
        } else {
            filteredKeys = sortedDerivedKeys.filter {
                appState.userData.selectedNetworks
                    .map(\.key)
                    .contains($0.network.networkSpecsKey)
            }
        }
        derivedKeys = filteredKeys
            .map {
                let details = "\($0.key.addressKey)\n\($0.network.networkSpecsKey)"
                return DerivedKeyRowModel(
                    viewModel: DerivedKeyRowViewModel($0.key),
                    actionModel: DerivedKeyActionModel(
                        tapAction: .init(action: .selectKey, details: details)
                    )
                )
            }
    }

    func refreshKeySummary() {
        guard let keysData = keysData else { return }
        keySummary = KeySummaryViewModel(
            keyName: keysData.root?.address.seedName ?? "",
            base58: keysData.root?.base58 ?? ""
        )
        removeSeed = keysData.root?.address.seedName ?? ""
    }
}
