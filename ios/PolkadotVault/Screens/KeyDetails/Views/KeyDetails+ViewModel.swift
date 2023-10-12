//
//  KeyDetails+ViewModel.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 27/10/2022.
//

import Combine
import Foundation
import SwiftUI

extension KeyDetailsView {
    enum OnCompletionAction: Equatable {
        case keySetDeleted
    }

    enum ViewState {
        case emptyState
        case list
    }

    final class ViewModel: ObservableObject {
        let keyDetailsService: KeyDetailsService
        private let networksService: GetManagedNetworksService
        private let warningStateMediator: WarningStateMediator
        private let cancelBag = CancelBag()

        private let exportPrivateKeyService: PrivateKeyQRCodeService
        private let keyDetailsActionsService: KeyDetailsActionService
        private let seedsMediator: SeedsMediating
        private var appState: AppState

        @Published var keyName: String
        @Published var keysData: MKeysNew?
        @Published var shouldPresentRemoveConfirmationModal = false
        @Published var shouldPresentBackupModal = false
        @Published var shouldPresentExportKeysSelection = false
        @Published var isShowingActionSheet = false
        @Published var isShowingRemoveConfirmation = false
        @Published var isShowingBackupModal = false
        @Published var isPresentingConnectivityAlert = false
        @Published var isPresentingExportKeySelection = false
        @Published var isPresentingRootDetails = false
        @Published var isPresentingKeyDetails = false
        @Published var presentedKeyDetails: MKeyDetails!
        @Published var presentedPublicKeyDetails: String!

        @Published var isShowingKeysExportModal = false
        @Published var isPresentingNetworkSelection = false
        @Published var isPresentingKeySetSelection = false
        @Published var isShowingRecoverKeySet = false
        @Published var isShowingCreateKeySet = false
        @Published var isPresentingSettings = false
        @Published var isPresentingQRScanner: Bool = false

        @Published var keySummary: KeySummaryViewModel?
        @Published var derivedKeys: [DerivedKeyRowModel] = []
        @Published var isFilteringActive: Bool = false
        // Error handling
        @Published var isPresentingError: Bool = false
        @Published var presentableError: ErrorBottomModalViewModel = .noNetworksAvailable()
        @Published var viewState: ViewState = .list
        @Published var backupModal: BackupModalViewModel?
        var snackbarViewModel: SnackbarViewModel = .init(title: "")
        @Published var isSnackbarPresented: Bool = false

        // Derive New Key
        @Published var isPresentingDeriveNewKey: Bool = false

        var keysExportModalViewModel: (() -> ExportMultipleKeysModalViewModel)?

        private let onDeleteCompletion: () -> Void
        /// Name of seed to be removed with `Remove Seed` action
        private var removeSeed: String = ""

        init(
            initialKeyName: String,
            onDeleteCompletion: @escaping () -> Void,
            exportPrivateKeyService: PrivateKeyQRCodeService = PrivateKeyQRCodeService(),
            keyDetailsService: KeyDetailsService = KeyDetailsService(),
            networksService: GetManagedNetworksService = GetManagedNetworksService(),
            keyDetailsActionsService: KeyDetailsActionService = KeyDetailsActionService(),
            warningStateMediator: WarningStateMediator = ServiceLocator.warningStateMediator,
            appState: AppState = ServiceLocator.appState,
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator
        ) {
            self.onDeleteCompletion = onDeleteCompletion
            self.exportPrivateKeyService = exportPrivateKeyService
            self.keyDetailsService = keyDetailsService
            self.networksService = networksService
            self.keyDetailsActionsService = keyDetailsActionsService
            self.warningStateMediator = warningStateMediator
            self.appState = appState
            self.seedsMediator = seedsMediator
            _keyName = .init(initialValue: initialKeyName)
            use(appState: appState)
            subscribeToNetworkChanges()
        }

        func use(appState _: AppState) {
            $isPresentingNetworkSelection.sink { newValue in
                guard !newValue else { return }
                self.isFilteringActive = !self.appState.userData.selectedNetworks.isEmpty
            }
            .store(in: cancelBag)
        }

        func onAppear() {
            refreshData()
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

        func onRemoveKeySetConfirmationTap() {
            let isRemoved = seedsMediator.removeSeed(seedName: removeSeed)
            guard isRemoved else { return }
            keyDetailsActionsService.forgetKeySet(seedName: keyName) { result in
                switch result {
                case .success:
                    self.snackbarViewModel = .init(
                        title: Localizable.KeySetsModal.Confirmation.snackbar.string,
                        style: .warning
                    )
                    self.isSnackbarPresented = true
                    if self.seedsMediator.seedNames.isEmpty {
                        self.onDeleteCompletion()
                    } else {
                        self.keyName = self.seedsMediator.seedNames.first ?? ""
                        self.refreshData()
                    }
                case let .failure(error):
                    self.presentableError = .alertError(message: error.localizedDescription)
                    self.isPresentingError = true
                }
            }
        }

        func onPublicKeyCompletion(_ completionAction: KeyDetailsPublicKeyView.OnCompletionAction) {
            switch completionAction {
            case .derivedKeyDeleted:
                refreshData()
                snackbarViewModel = .init(
                    title: Localizable.PublicKeyDetailsModal.Confirmation.snackbar.string,
                    style: .warning
                )
                isSnackbarPresented = true
            }
        }

        func onAddDerivedKeyCompletion(_ completionAction: CreateKeyNetworkSelectionView.OnCompletionAction) {
            switch completionAction {
            case .derivedKeyCreated:
                refreshData()
                snackbarViewModel = .init(
                    title: Localizable.CreateDerivedKey.Snackbar.created.string,
                    style: .info
                )
                isSnackbarPresented = true
            }
        }

        func createDerivedKeyViewModel() -> CreateKeyNetworkSelectionView.ViewModel {
            .init(
                seedName: keysData?.root?.address.seedName ?? "",
                keyName: keyName,
                // swiftlint: disable:next force_unwrapping
                keySet: keysData!,
                onCompletion: onAddDerivedKeyCompletion(_:)
            )
        }

        func exportSelectedKeys() {
            isShowingKeysExportModal = true
        }

        func onExportKeySelectionComplete(_ completionAction: ExportKeysSelectionModal.OnCompletionAction) {
            switch completionAction {
            case .onCancel:
                ()
            case let .onKeysExport(selectedKeys):
                guard let keySummary else { return }
                let derivedKeys = selectedKeys.map {
                    DerivedKeyExportModel(viewModel: $0.viewModel, keyData: $0.keyData)
                }
                keysExportModalViewModel = { ExportMultipleKeysModalViewModel(
                    key: keySummary,
                    derivedKeys: derivedKeys,
                    count: selectedKeys.count
                )
                }
                DispatchQueue.main.asyncAfter(deadline: .now()) {
                    self.isShowingKeysExportModal = true
                }
            }
        }

        func onKeySetSelectionComplete(_ completionAction: ManageKeySetsView.OnCompletionAction) {
            switch completionAction {
            case .onClose:
                ()
            case .addKeySet:
                DispatchQueue.main.async {
                    self.isShowingCreateKeySet = true
                }
            case .recoverKeySet:
                DispatchQueue.main.async {
                    self.isShowingRecoverKeySet = true
                }
            case let .viewKeySet(selectedKeySet):
                isFilteringActive = false
                appState.userData.selectedNetworks = []
                keyName = selectedKeySet.seedName
                refreshData()
            }
        }

        func onKeySetAddCompletion(_ completionAction: CreateKeysForNetworksView.OnCompletionAction) {
            let message: String
            switch completionAction {
            case let .createKeySet(seedName):
                message = Localizable.CreateKeysForNetwork.Snackbar.keySetCreated(seedName)
                keyName = seedName
            case let .recoveredKeySet(seedName):
                message = Localizable.CreateKeysForNetwork.Snackbar.keySetRecovered(seedName)
                keyName = seedName
            }
            refreshData()
            snackbarViewModel = .init(
                title: message,
                style: .info
            )
            isSnackbarPresented = true
        }
    }
}

// MARK: - Tap Actions

extension KeyDetailsView.ViewModel {
    func onCreateDerivedKeyTap() {
        if appState.userData.allNetworks.isEmpty {
            presentableError = .noNetworksAvailable()
            isPresentingError = true
        } else {
            isPresentingDeriveNewKey = true
        }
    }

    func onRootKeyTap() {
        isPresentingRootDetails = true
    }

    func onSettingsTap() {
        isPresentingSettings = true
    }

    func onMoreTap() {
        isShowingActionSheet = true
    }

    func onQRScannerTap() {
        isPresentingQRScanner = true
    }

    func onKeySetSelectionTap() {
        isPresentingKeySetSelection = true
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
        DispatchQueue.main.async {
            self.keyDetailsActionsService.navigateToPublicKey(
                addressKey: deriveKey.keyData.key.addressKey,
                networkSpecsKey: deriveKey.keyData.network.networkSpecsKey
            ) { result in
                switch result {
                case let .success(keyDetails):
                    self.presentedPublicKeyDetails = deriveKey.addressKey
                    self.presentedKeyDetails = keyDetails
                    self.isPresentingKeyDetails = true
                case let .failure(error):
                    self.presentableError = .alertError(message: error.localizedDescription)
                    self.isPresentingError = true
                }
            }
        }
    }

    func onConnectivityAlertTap() {
        warningStateMediator.resetConnectivityWarnings()
        shouldPresentBackupModal.toggle()
    }
}

// MARK: - Modals

extension KeyDetailsView.ViewModel {
    func onQRScannerDismiss() {
        refreshData()
    }

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
                keyDetailsActionsService.performBackupSeed(seedName: keyName) { result in
                    switch result {
                    case .success:
                        self.updateBackupModel()
                        self.isShowingBackupModal = true
                    case let .failure(error):
                        self.presentableError = .alertError(message: error.localizedDescription)
                        self.isPresentingError = true
                    }
                }
            }
        }
        if shouldPresentExportKeysSelection {
            shouldPresentExportKeysSelection = false
            isPresentingExportKeySelection = true
        }
    }

    func clearBackupModalState() {
        backupModal = nil
    }

    func rootKeyDetails() -> RootKeyDetailsModal.Renderable {
        .init(
            seedName: keyName,
            identicon: keysData?.root?.address.identicon,
            base58: keysData?.root?.base58 ?? ""
        )
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
        guard let keysData else { return }
        let sortedDerivedKeys = keysData.set
            .sorted(by: { $0.key.address.path < $1.key.address.path })
        let filteredKeys: [MKeyAndNetworkCard] = if appState.userData.selectedNetworks.isEmpty {
            sortedDerivedKeys
        } else {
            sortedDerivedKeys.filter {
                appState.userData.selectedNetworks
                    .map(\.key)
                    .contains($0.network.networkSpecsKey)
            }
        }
        derivedKeys = filteredKeys
            .map {
                DerivedKeyRowModel(
                    keyData: $0,
                    viewModel: DerivedKeyRowViewModel($0),
                    addressKey: $0.key.addressKey
                )
            }
        viewState = derivedKeys.isEmpty ? .emptyState : .list
    }

    func refreshKeySummary() {
        guard let keysData else { return }
        keySummary = KeySummaryViewModel(
            keyName: keysData.root?.address.seedName ?? "",
            base58: keysData.root?.base58 ?? ""
        )
        removeSeed = keysData.root?.address.seedName ?? ""
    }
}
