//
//  NetworkSettingsDetails.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 20/12/2022.
//

import SwiftUI

struct NetworkSettingsDetails: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var navigation: NavigationCoordinator

    var body: some View {
        VStack(spacing: 0) {
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    leftButton: .arrow,
                    rightButton: .more,
                    backgroundColor: Asset.backgroundSystem.swiftUIColor
                ),
                actionModel: .init(rightBarMenuAction: viewModel.onMoreMenuTap)
            )
            ScrollView {
                VStack(alignment: .leading, spacing: 0) {
                    VStack(alignment: .center, spacing: 0) {
                        NetworkLogoIcon(logo: viewModel.networkDetails.logo, size: Heights.networkLogoInHeader)
                            .padding(.bottom, Spacing.small)
                        Text(viewModel.networkDetails.title)
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
            }
            .background(Asset.backgroundPrimary.swiftUIColor)
            .onAppear {
                viewModel.use(navigation: navigation)
            }
            .fullScreenCover(isPresented: $viewModel.isPresentingRemoveMetadataConfirmation) {
                HorizontalActionsBottomModal(
                    viewModel: .removeMetadata,
                    mainAction: viewModel.removeMetadata(),
                    dismissAction: viewModel.cancelMetadataRemoval(),
                    isShowingBottomAlert: $viewModel.isPresentingRemoveMetadataConfirmation
                )
                .clearModalBackground()
            }
            .fullScreenCover(isPresented: $viewModel.isPresentingRemoveNetworkConfirmation) {
                HorizontalActionsBottomModal(
                    viewModel: .removeNetwork,
                    mainAction: viewModel.removeNetwork(),
                    dismissAction: viewModel.cancelNetworkRemoval(),
                    isShowingBottomAlert: $viewModel.isPresentingRemoveNetworkConfirmation
                )
                .clearModalBackground()
            }
            .fullScreenCover(
                isPresented: $viewModel.isShowingActionSheet,
                onDismiss: { viewModel.onMoreActionSheetDismissal() }
            ) {
                NetworkSettingsDetailsActionModal(
                    isShowingActionSheet: $viewModel.isShowingActionSheet,
                    shouldPresentRemoveNetworkConfirmation: $viewModel.shouldPresentRemoveNetworkConfirmation,
                    shouldSignSpecs: $viewModel.shouldSignSpecs
                )
                .clearModalBackground()
            }
        }
    }
}

private extension NetworkSettingsDetails {
    @ViewBuilder
    func networkSpecs() -> some View {
        VStack(alignment: .leading, spacing: Spacing.small) {
            rowWrapper(
                Localizable.Settings.NetworkDetails.Label.name.string,
                viewModel.networkDetails.name
            )
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
            switch viewModel.networkDetails.currentVerifier.ttype {
            case "general":
                generalVerifier(viewModel.networkDetails.currentVerifier)
            case "custom":
                customVerifier(viewModel.networkDetails.currentVerifier)
            case "none":
                rowWrapper(
                    Localizable.Settings.NetworkDetails.Label.verifier.string,
                    Localizable.Settings.NetworkDetails.Label.Verifier.none.string,
                    isLast: true
                )
            default:
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
            Identicon(identicon: verifier.details.identicon)
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
    final class ViewModel: ObservableObject {
        private weak var navigation: NavigationCoordinator!
        private let snackbarPresentation: BottomSnackbarPresentation
        private var metadataToDelete: MMetadataRecord?

        @Published var isPresentingRemoveMetadataConfirmation = false
        @Published var networkDetails: MNetworkDetails
        @Published var shouldSignSpecs = false
        @Published var isShowingActionSheet = false
        @Published var shouldPresentRemoveNetworkConfirmation = false
        @Published var isPresentingRemoveNetworkConfirmation = false

        init(
            networkDetails: MNetworkDetails,
            snackbarPresentation: BottomSnackbarPresentation = ServiceLocator.bottomSnackbarPresentation
        ) {
            _networkDetails = .init(wrappedValue: networkDetails)
            self.snackbarPresentation = snackbarPresentation
        }

        func removeMetadata() {
            isPresentingRemoveMetadataConfirmation = false
            if case let .nNetworkDetails(updatedDetails) = navigation
                .performFake(navigation: .init(action: .removeMetadata)).screenData {
                networkDetails = updatedDetails
                snackbarPresentation.viewModel = .init(
                    title: Localizable.Settings.NetworkDetails.DeleteMetadata.Label
                        .confirmation(metadataToDelete?.specsVersion ?? ""),
                    style: .warning
                )
                snackbarPresentation.isSnackbarPresented = true
                metadataToDelete = nil
            }
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func onBackTap() {
            navigation.perform(navigation: .init(action: .goBack))
        }

        func onAddTap() {
            navigation.shouldPresentQRScanner = true
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.2) {
                self.navigation.performFake(navigation: .init(action: .goBack))
                self.navigation.perform(navigation: .init(action: .goBack))
                self.navigation.performFake(navigation: .init(action: .navbarScan))
            }
        }

        func didTapDelete(_ metadata: MMetadataRecord) {
            metadataToDelete = metadata
            navigation.performFake(navigation: .init(action: .manageMetadata, details: metadata.specsVersion))
            isPresentingRemoveMetadataConfirmation = true
        }

        func didTapSign(_ metadata: MMetadataRecord) {
            navigation.performFake(navigation: .init(action: .manageMetadata, details: metadata.specsVersion))
            navigation.perform(navigation: .init(action: .signMetadata))
        }

        func cancelMetadataRemoval() {
            metadataToDelete = nil
            isPresentingRemoveMetadataConfirmation = false
            navigation.performFake(navigation: .init(action: .goBack))
        }

        func onMoreMenuTap() {
            navigation.performFake(navigation: .init(action: .rightButtonAction))
            isShowingActionSheet = true
        }

        func onMoreActionSheetDismissal() {
            if shouldSignSpecs {
                navigation.perform(navigation: .init(action: .signNetworkSpecs))
            }
            if shouldPresentRemoveNetworkConfirmation {
                shouldPresentRemoveNetworkConfirmation = false
                isPresentingRemoveNetworkConfirmation = true
            }
        }

        func removeNetwork() {
            snackbarPresentation.viewModel = .init(
                title: Localizable.Settings.NetworkDetails.DeleteNetwork.Label
                    .confirmation(networkDetails.title),
                style: .warning
            )
            snackbarPresentation.isSnackbarPresented = true
            navigation.perform(navigation: .init(action: .removeNetwork))
        }

        func cancelNetworkRemoval() {
            isPresentingRemoveNetworkConfirmation = false
            navigation.performFake(navigation: .init(action: .goBack))
        }
    }
}

struct NetworkSettingsDetails_Previews: PreviewProvider {
    static var previews: some View {
        NetworkSelectionSettings(
            viewModel: .init(networks: [])
        )
        .environmentObject(NavigationCoordinator())
    }
}
