//
//  NetworkSettingsDetails.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 20/12/2022.
//

import Combine
import SwiftUI

struct NetworkSettingsDetails: View {
    @StateObject var viewModel: ViewModel
    @Environment(\.presentationMode) var presentationMode

    var body: some View {
        VStack(spacing: 0) {
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    leftButtons: [.init(
                        type: .arrow,
                        action: {
                            presentationMode.wrappedValue.dismiss()
                        }
                    )],
                    rightButtons: [.init(type: .more, action: viewModel.onMoreMenuTap)],
                    backgroundColor: Asset.backgroundPrimary.swiftUIColor
                )
            )
            ScrollView(showsIndicators: false) {
                VStack(alignment: .leading, spacing: 0) {
                    VStack(alignment: .center, spacing: 0) {
                        NetworkLogoIcon(networkName: viewModel.networkDetails.logo, size: Heights.networkLogoInHeader)
                            .padding(.bottom, Spacing.small)
                        Text(viewModel.networkDetails.name.capitalized)
                            .font(PrimaryFont.titleM.font)
                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                            .padding(.bottom, Spacing.large)
                        HStack {
                            Spacer()
                        }
                    }
                    // Network Specs
                    Localizable.Settings.NetworkDetails.Label.specs.text
                        .font(PrimaryFont.bodyL.font)
                        .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                        .padding(.leading, Spacing.large)
                        .padding(.bottom, Spacing.extraSmall)
                    networkSpecs()
                        .verticalRoundedBackgroundContainer()
                        .padding(.horizontal, Spacing.extraSmall)
                        .font(PrimaryFont.bodyL.font)
                    // Metadata
                    if !viewModel.networkDetails.meta.isEmpty {
                        Localizable.Settings.NetworkDetails.Label.metadata.text
                            .font(PrimaryFont.bodyL.font)
                            .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                            .padding(.top, Spacing.large)
                            .padding(.leading, Spacing.large)
                            .padding(.bottom, Spacing.extraSmall)
                        VStack(spacing: Spacing.small) {
                            ForEach(viewModel.networkDetails.meta, id: \.metaHash) {
                                metadata($0)
                                    .padding(.horizontal, Spacing.extraSmall)
                            }
                        }
                    }
                    HStack(alignment: .center, spacing: 0) {
                        Asset.add.swiftUIImage
                            .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                            .frame(width: Heights.networkLogoInCell, height: Heights.networkLogoInCell)
                            .background(Circle().foregroundColor(Asset.accentPink12.swiftUIColor))
                            .padding(.trailing, Spacing.small)
                        Text(Localizable.Settings.NetworkDetails.Action.add.string)
                            .foregroundColor(Asset.accentPink.swiftUIColor)
                            .font(PrimaryFont.labelL.font)
                        Spacer()
                    }
                    .contentShape(Rectangle())
                    .padding(.top, Spacing.large)
                    .padding(.horizontal, Spacing.medium)
                    .frame(height: Heights.networkSelectionSettings)
                    .onTapGesture {
                        viewModel.onAddTap()
                    }
                    Spacer()
                        .frame(height: Spacing.large)
                }
                NavigationLink(
                    destination: SignSpecsListView(
                        viewModel: .init(
                            networkKey: viewModel.networkKey,
                            content: viewModel.signSpecList
                        )
                    )
                    .navigationBarHidden(true),
                    isActive: $viewModel.isPresentingSignSpecList
                ) { EmptyView() }
            }
            .background(Asset.backgroundPrimary.swiftUIColor)
            .onReceive(viewModel.dismissViewRequest) { _ in
                presentationMode.wrappedValue.dismiss()
            }
            .fullScreenModal(isPresented: $viewModel.isPresentingRemoveMetadataConfirmation) {
                HorizontalActionsBottomModal(
                    viewModel: .removeMetadata,
                    mainAction: viewModel.removeMetadata(),
                    dismissAction: viewModel.cancelMetadataRemoval(),
                    isShowingBottomAlert: $viewModel.isPresentingRemoveMetadataConfirmation
                )
                .clearModalBackground()
            }
            .fullScreenModal(isPresented: $viewModel.isPresentingRemoveNetworkConfirmation) {
                HorizontalActionsBottomModal(
                    viewModel: .removeNetwork,
                    mainAction: viewModel.removeNetwork(),
                    dismissAction: viewModel.cancelNetworkRemoval(),
                    isShowingBottomAlert: $viewModel.isPresentingRemoveNetworkConfirmation
                )
                .clearModalBackground()
            }
            .fullScreenModal(
                isPresented: $viewModel.isShowingActionSheet,
                onDismiss: {
                    // iOS 15 handling of following .fullscreen presentation after dismissal, we need to dispatch this
                    // async
                    DispatchQueue.main.async { viewModel.onMoreActionSheetDismissal() }
                }
            ) {
                NetworkSettingsDetailsActionModal(
                    isShowingActionSheet: $viewModel.isShowingActionSheet,
                    shouldPresentRemoveNetworkConfirmation: $viewModel.shouldPresentRemoveNetworkConfirmation,
                    shouldSignSpecs: $viewModel.shouldSignSpecs
                )
                .clearModalBackground()
            }
            .fullScreenModal(
                isPresented: $viewModel.isShowingQRScanner,
                onDismiss: viewModel.onQRScannerDismiss
            ) {
                CameraView(
                    viewModel: .init(
                        isPresented: $viewModel.isShowingQRScanner
                    )
                )
            }
            .bottomSnackbar(
                viewModel.snackbarViewModel,
                isPresented: $viewModel.isSnackbarPresented
            )
            .navigationBarHidden(true)
        }
    }
}

private extension NetworkSettingsDetails {
    @ViewBuilder
    func networkSpecs() -> some View {
        VStack(alignment: .leading, spacing: Spacing.small) {
            rowWrapper(
                Localizable.Settings.NetworkDetails.Label.basePrefix.string,
                String(viewModel.networkDetails.base58prefix)
            )
            rowWrapper(
                Localizable.Settings.NetworkDetails.Label.decimals.string,
                String(viewModel.networkDetails.decimals)
            )
            rowWrapper(
                Localizable.Settings.NetworkDetails.Label.unit.string,
                viewModel.networkDetails.unit
            )
            verticalRowWrapper(
                Localizable.Settings.NetworkDetails.Label.genesisHash.string,
                viewModel.networkDetails.genesisHash.formattedAsString
            )
            switch viewModel.networkDetails.currentVerifier.type {
            case .general:
                generalVerifier(viewModel.networkDetails.currentVerifier)
            case .custom:
                customVerifier(viewModel.networkDetails.currentVerifier)
            case .none:
                rowWrapper(
                    Localizable.Settings.NetworkDetails.Label.verifier.string,
                    Localizable.Settings.NetworkDetails.Label.Verifier.none.string,
                    isLast: true
                )
            case .unknown:
                rowWrapper(
                    Localizable.Settings.NetworkDetails.Label.verifier.string,
                    Localizable.Settings.NetworkDetails.Label.Verifier.unknown.string,
                    isLast: true
                )
            }
        }
    }

    @ViewBuilder
    func metadata(_ metadata: MMetadataRecord) -> some View {
        VStack(alignment: .leading, spacing: Spacing.small) {
            rowWrapper(
                Localizable.Settings.NetworkDetails.Label.version.string,
                metadata.specsVersion
            )
            verticalRowWrapper(
                Localizable.Settings.NetworkDetails.Label.hash.string,
                metadata.metaHash
            )
            HStack {
                Localizable.Settings.NetworkDetails.Action.sign.text
                    .foregroundColor(Asset.accentPink300.swiftUIColor)
                Spacer()
                Asset.chevronRight.swiftUIImage
                    .foregroundColor(Asset.textAndIconsDisabled.swiftUIColor)
                    .padding(.trailing, Spacing.extraSmall)
            }
            .contentShape(Rectangle())
            .onTapGesture {
                viewModel.didTapSign(metadata)
            }
            Divider()
            HStack {
                Localizable.Settings.NetworkDetails.Action.delete.text
                    .foregroundColor(Asset.accentRed300.swiftUIColor)
                Spacer()
            }
            .contentShape(Rectangle())
            .onTapGesture {
                viewModel.didTapDelete(metadata)
            }
        }
        .verticalRoundedBackgroundContainer()
    }
}

private extension NetworkSettingsDetails {
    @ViewBuilder
    func rowWrapper(
        _ key: String,
        _ value: String,
        isLast: Bool = false
    ) -> some View {
        HStack {
            Text(key)
                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
            Spacer()
            Text(value)
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
        }
        if !isLast {
            Divider()
        }
    }

    @ViewBuilder
    func verticalRowWrapper(
        _ key: String,
        _ value: String,
        isLast: Bool = false
    ) -> some View {
        VStack(alignment: .leading, spacing: Spacing.extraSmall) {
            Text(key)
                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
            Text(value)
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
            if !isLast {
                Divider()
            }
        }
    }
}

private extension NetworkSettingsDetails {
    @ViewBuilder
    func generalVerifier(_ verifier: MVerifier) -> some View {
        rowWrapper(
            Localizable.Settings.NetworkDetails.Label.verifier.string,
            Localizable.Settings.NetworkDetails.Label.Verifier.general.string
        )
        verticalRowWrapper(
            Localizable.Settings.NetworkDetails.Label.Verifier.key.string,
            verifier.details.publicKey
        )
        rowWrapper(
            Localizable.Settings.NetworkDetails.Label.Verifier.crypto.string,
            verifier.details.encryption,
            isLast: true
        )
    }

    @ViewBuilder
    func customVerifier(_ verifier: MVerifier) -> some View {
        rowWrapper(
            Localizable.Settings.NetworkDetails.Label.verifier.string,
            Localizable.Settings.NetworkDetails.Label.Verifier.custom.string
        )
        HStack {
            Localizable.Settings.NetworkDetails.Label.Verifier.identicon.text
                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
            Spacer()
            IdenticonView(identicon: verifier.details.identicon)
        }
        Divider()
        verticalRowWrapper(
            Localizable.Settings.NetworkDetails.Label.Verifier.publicKey.string,
            verifier.details.publicKey
        )
        rowWrapper(
            Localizable.Settings.NetworkDetails.Label.Verifier.crypto.string,
            verifier.details.encryption,
            isLast: true
        )
    }
}

extension NetworkSettingsDetails {
    enum OnCompletionAction: Equatable {
        case networkDeleted(String)
    }

    final class ViewModel: ObservableObject {
        private let cancelBag = CancelBag()
        private let networkDetailsService: ManageNetworkDetailsService
        let networkKey: String
        private var metadataToDelete: MMetadataRecord?

        var dismissViewRequest: AnyPublisher<Void, Never> { dismissRequest.eraseToAnyPublisher() }
        private let dismissRequest = PassthroughSubject<Void, Never>()
        private let onCompletion: (OnCompletionAction) -> Void
        @Published var isPresentingRemoveMetadataConfirmation = false
        @Published var networkDetails: MNetworkDetails
        @Published var shouldSignSpecs = false
        @Published var isShowingActionSheet = false
        @Published var shouldPresentRemoveNetworkConfirmation = false
        @Published var isPresentingRemoveNetworkConfirmation = false

        @Published var signSpecList: MSignSufficientCrypto!
        @Published var isPresentingSignSpecList: Bool = false
        @Published var isShowingQRScanner: Bool = false
        var snackbarViewModel: SnackbarViewModel = .init(title: "")
        @Published var isSnackbarPresented: Bool = false

        init(
            networkKey: String,
            networkDetails: MNetworkDetails,
            networkDetailsService: ManageNetworkDetailsService = ManageNetworkDetailsService(),
            onCompletion: @escaping (OnCompletionAction) -> Void
        ) {
            self.networkKey = networkKey
            self.networkDetailsService = networkDetailsService
            self.onCompletion = onCompletion
            _networkDetails = .init(initialValue: networkDetails)
            listenToNavigationUpdates()
        }

        func removeMetadata() {
            isPresentingRemoveMetadataConfirmation = false
            networkDetails = networkDetailsService.deleteNetworkMetadata(
                networkKey,
                metadataToDelete?.specsVersion ?? ""
            )
            snackbarViewModel = .init(
                title: Localizable.Settings.NetworkDetails.DeleteMetadata.Label
                    .confirmation(metadataToDelete?.specsVersion ?? ""),
                style: .warning
            )
            isSnackbarPresented = true
            metadataToDelete = nil
        }

        func onAddTap() {
            isShowingQRScanner = true
        }

        func onQRScannerDismiss() {
            updateView()
        }

        func didTapDelete(_ metadata: MMetadataRecord) {
            metadataToDelete = metadata
            isPresentingRemoveMetadataConfirmation = true
        }

        func didTapSign(_ metadata: MMetadataRecord) {
            signSpecList = networkDetailsService.signMetadataSpecList(networkKey, metadata.specsVersion)
            isPresentingSignSpecList = true
        }

        func cancelMetadataRemoval() {
            metadataToDelete = nil
            isPresentingRemoveMetadataConfirmation = false
        }

        func onMoreMenuTap() {
            isShowingActionSheet = true
        }

        func onMoreActionSheetDismissal() {
            if shouldSignSpecs {
                signSpecList = networkDetailsService.signSpecList(networkKey)
                shouldSignSpecs = false
                isPresentingSignSpecList = true
            }
            if shouldPresentRemoveNetworkConfirmation {
                shouldPresentRemoveNetworkConfirmation = false
                isPresentingRemoveNetworkConfirmation = true
            }
        }

        func removeNetwork() {
            networkDetailsService.deleteNetwork(networkKey)
            dismissRequest.send()
            onCompletion(.networkDeleted(networkDetails.title))
        }

        func cancelNetworkRemoval() {
            isPresentingRemoveNetworkConfirmation = false
        }

        private func updateView() {
            guard let updatedNetworkDetails = networkDetailsService.refreshCurrentNavigationState(networkKey)
            else { return }
            networkDetails = updatedNetworkDetails
        }

        private func listenToNavigationUpdates() {
            guard cancelBag.subscriptions.isEmpty else { return }
            $isPresentingSignSpecList.sink { [weak self] isPresentingSignSpecList in
                guard let self = self, !isPresentingSignSpecList else { return }
                self.signSpecList = nil
                self.updateView()
            }.store(in: cancelBag)
        }
    }
}

#if DEBUG
    struct NetworkSettingsDetails_Previews: PreviewProvider {
        static var previews: some View {
            NetworkSelectionSettings(
                viewModel: .init()
            )
        }
    }
#endif
