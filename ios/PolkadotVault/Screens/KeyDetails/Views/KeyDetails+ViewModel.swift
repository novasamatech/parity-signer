//
//  KeyDetails+ViewModel.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 27/10/2022.
//

import Foundation

extension KeyDetailsView {
    enum ViewState {
        case emptyState
        case list
    }

    final class ViewModel: ObservableObject {
        let keyDetailsService: KeyDetailsService
        private let networksService: GetAllNetworksService
        private let warningStateMediator: WarningStateMediator
        private let cancelBag = CancelBag()
        private let forgetKeyActionHandler: ForgetKeySetAction
        private let exportPrivateKeyService: PrivateKeyQRCodeService

        let keyName: String
        /// `MKwysNew` will currently be `nil` when navigating through given navigation path:
        /// `.newSeed` -> `.keys`, data will be filled on `onAppear`, so this can remain optional
        var keysData: MKeysNew?
        private var appState: AppState
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
        @Published var isPresentingKeyDetails = false
        @Published var presentedKeyDetails: MKeyDetails!

        @Published var isShowingKeysExportModal = false
        // Network selection
        @Published var isPresentingNetworkSelection = false

        @Published var keySummary: KeySummaryViewModel?
        @Published var derivedKeys: [DerivedKeyRowModel] = []
        @Published var selectedKeys: [DerivedKeyRowModel] = []
        @Published var isFilteringActive: Bool = false
        // Error handling
        @Published var isPresentingError: Bool = false
        @Published var presentableError: ErrorBottomModalViewModel = .noNetworksAvailable()
        @Published var viewState: ViewState = .list
        @Published var backupModal: BackupModalViewModel?

        /// Name of seed to be removed with `Remove Seed` action
        private var removeSeed: String = ""

        init(
            keyName: String,
            exportPrivateKeyService: PrivateKeyQRCodeService = PrivateKeyQRCodeService(),
            keyDetailsService: KeyDetailsService = KeyDetailsService(),
            networksService: GetAllNetworksService = GetAllNetworksService(),
            forgetKeyActionHandler: ForgetKeySetAction = ForgetKeySetAction(),
            warningStateMediator: WarningStateMediator = ServiceLocator.warningStateMediator,
            appState: AppState = ServiceLocator.appState
        ) {
            self.keyName = keyName
            self.exportPrivateKeyService = exportPrivateKeyService
            self.keyDetailsService = keyDetailsService
            self.networksService = networksService
            self.forgetKeyActionHandler = forgetKeyActionHandler
            self.warningStateMediator = warningStateMediator
            self.appState = appState
            use(appState: appState)
            updateRenderables()
            subscribeToNetworkChanges()
        }

        func use(appState: AppState) {
            keysData = appState.userData.keysData
            $isPresentingNetworkSelection.sink { newValue in
                guard !newValue else { return }
                self.isFilteringActive = !self.appState.userData.selectedNetworks.isEmpty
            }
            .store(in: cancelBag)
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
            forgetKeyActionHandler.use(navigation: navigation)
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
                switch result {
                case let .success(keysData):
                    self.appState.userData.keysData = keysData
                    self.keysData = keysData
                    self.updateRenderables()
                case let .failure(error):
                    self.presentableError = .alertError(message: error.description)
                    self.isPresentingError = true
                }
            }
        }

        func refreshNetworks() {
            networksService.getNetworks { result in
                switch result {
                case let .success(networks):
                    self.appState.userData.allNetworks = networks
                case let .failure(error):
                    self.presentableError = .alertError(message: error.description)
                    self.isPresentingError = true
                }
            }
        }

        func onBackTap() {
            appState.userData.keysData = nil
            navigation.performFake(navigation: .init(action: .goBack))
        }

        func onRemoveKeySetConfirmationTap() {
            forgetKeyActionHandler.forgetKeySet(removeSeed)
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
            if selectedKeys.contains(deriveKey) {
                selectedKeys.removeAll { $0 == deriveKey }
            } else {
                selectedKeys.append(deriveKey)
            }
        } else {
            guard case let .keyDetails(keyDetails) = navigation
                .performFake(navigation: .init(action: .selectKey, details: deriveKey.publicKeyDetails)).screenData,
                let keyDetails = keyDetails else { return }
            presentedKeyDetails = keyDetails
            isPresentingKeyDetails = true
        }
    }

    func onConnectivityAlertTap() {
        warningStateMediator.resetConnectivityWarnings()
        shouldPresentBackupModal.toggle()
    }
}

// MARK: - Modals

extension KeyDetailsView.ViewModel {
    func onActionSheetDismissal() {
        let isAlertVisible = warningStateMediator.alert
        if shouldPresentRemoveConfirmationModal {
            shouldPresentRemoveConfirmationModal.toggle()
            isShowingRemoveConfirmation.toggle()
        }
        if shouldPresentBackupModal {
            shouldPresentBackupModal.toggle()
            if isAlertVisible {
                isPresentingConnectivityAlert.toggle()
            } else {
                updateBackupModel()
                isShowingBackupModal = true
            }
        }
        if shouldPresentSelectionOverlay {
            shouldPresentSelectionOverlay.toggle()
            isPresentingSelectionOverlay.toggle()
        }
    }

    func clearBackupModalState() {
        backupModal = nil
    }

    func keyExportModel() -> ExportMultipleKeysModalViewModel? {
        guard let keySummary = keySummary else { return nil }
        let derivedKeys = selectedKeys.map {
            DerivedKeyExportModel(viewModel: $0.viewModel, keyData: $0.keyData)
        }
        return ExportMultipleKeysModalViewModel(
            selectedItems: .keys(
                key: keySummary,
                derivedKeys: derivedKeys
            ),
            count: selectedKeys.count
        )
    }

    func rootKeyDetails() -> RootKeyDetailsModal.ViewModel {
        .init(name: keySummary?.keyName ?? "", publicKey: keySummary?.base58 ?? "")
    }
}

private extension KeyDetailsView.ViewModel {
    func updateBackupModel() {
        backupModal = exportPrivateKeyService.backupViewModel(keysData)
    }

    func keyData(for derivedKey: DerivedKeyRowModel) -> MKeyAndNetworkCard? {
        keysData?.set.first(where: { $0.key.address.path == derivedKey.viewModel.path })
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
                    keyData: $0,
                    viewModel: DerivedKeyRowViewModel($0),
                    publicKeyDetails: details
                )
            }
        viewState = derivedKeys.isEmpty ? .emptyState : .list
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
