//
//  CreateDerivedKeyView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 10/01/2023.
//

import SwiftUI

struct CreateDerivedKeyView: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var navigation: NavigationCoordinator
    @EnvironmentObject private var appState: AppState

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: Localizable.CreateDerivedKey.Label.title.string,
                    leftButtons: [.init(
                        type: .xmark,
                        action: { navigation.perform(navigation: .init(action: .goBack)) }
                    )],
                    rightButtons: [.init(type: .questionmark, action: viewModel.onRightNavigationButtonTap)],
                    backgroundColor: Asset.backgroundSystem.swiftUIColor
                )
            )
            VStack(alignment: .leading, spacing: 0) {
                Localizable.CreateDerivedKey.Label.Header.network.text
                    .font(PrimaryFont.bodyL.font)
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    .padding(.bottom, Spacing.medium)
                networkSelectionInput()
                    .padding(.bottom, Spacing.large)
                HStack(spacing: Spacing.extraExtraSmall) {
                    Localizable.CreateDerivedKey.Label.Header.path.text
                        .font(PrimaryFont.bodyL.font)
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    Asset.smallRoundQuestionmark.swiftUIImage
                        .foregroundColor(Asset.accentPink300.swiftUIColor)
                        .frame(width: Sizes.roundedQuestionmark, height: Sizes.roundedQuestionmark)
                }
                .containerShape(Rectangle())
                .onTapGesture {
                    viewModel.onDerivationPathQuestionTap()
                }
                .padding(.bottom, Spacing.medium)
                derivationPathInput()
                    .padding(.bottom, Spacing.small)
                HStack {
                    Localizable.CreateDerivedKey.Label.Footer.path.text
                        .font(PrimaryFont.captionM.font)
                        .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    Spacer()
                    Asset.infoIconBold.swiftUIImage
                        .foregroundColor(Asset.accentPink300.swiftUIColor)
                }
                Spacer()
                PrimaryButton(
                    action: viewModel.onCreateDerivedKeyTap,
                    text: Localizable.CreateDerivedKey.Action.add.key,
                    style: .primary(isDisabled: $viewModel.isActionDisabled)
                )
            }
            .padding(.horizontal, Spacing.large)
            .padding(.bottom, Spacing.large)
            .padding(.top, Spacing.medium)
        }
        .background(Asset.backgroundPrimary.swiftUIColor)
        .onAppear {
            viewModel.use(navigation: navigation)
            viewModel.use(appState: appState)
        }
        .fullScreenCover(
            isPresented: $viewModel.isPresentingInfoModal
        ) {
            ErrorBottomModal(
                viewModel: viewModel.presentableInfoModal,
                isShowingBottomAlert: $viewModel.isPresentingInfoModal
            )
            .clearModalBackground()
        }
        .fullScreenCover(
            isPresented: $viewModel.isPresentingNetworkSelection
        ) {
            ChooseNetworkForKeyView(
                viewModel: .init(
                    isPresented: $viewModel.isPresentingNetworkSelection,
                    networkSelection: $viewModel.networkSelection
                )
            )
            .clearModalBackground()
        }
        .fullScreenCover(
            isPresented: $viewModel.isPresentingDerivationPath
        ) {
            DerivationPathNameView(
                viewModel: .init(
                    seedName: viewModel.seedName,
                    derivationPath: $viewModel.derivationPath,
                    isPresented: $viewModel.isPresentingDerivationPath,
                    networkSelection: $viewModel.networkSelection
                )
            )
        }
        .fullScreenCover(
            isPresented: $viewModel.isPresentingConfirmation
        ) {
            CreateDerivedKeyConfirmationView(
                viewModel: .init(
                    isPresented: $viewModel.isPresentingConfirmation,
                    derivationPath: viewModel.unwrappedDerivationPath()
                )
            )
            .clearModalBackground()
        }
    }

    @ViewBuilder
    func networkSelectionInput() -> some View {
        HStack(spacing: 0) {
            Spacer()
                .frame(width: Spacing.medium)
            switch viewModel.networkSelection {
            case let .network(network):
                Localizable.CreateDerivedKey.Label.Network.single.text
                    .font(PrimaryFont.bodyL.font)
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                Spacer()
                HStack(spacing: 0) {
                    NetworkLogoIcon(networkName: network.logo, size: Heights.networkLogoInCapsule)
                        .padding(.trailing, Spacing.extraSmall)
                    Text(network.title.capitalized)
                        .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                        .font(PrimaryFont.bodyM.font)
                        .padding(.trailing, Spacing.extraSmall)
                }
                .padding(Spacing.minimal)
                .background(Asset.fill12.swiftUIColor)
                .clipShape(Capsule())
            case .allowedOnAnyNetwork:
                Localizable.CreateDerivedKey.Label.Network.onAny.text
                    .font(PrimaryFont.bodyL.font)
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                Spacer()
            }
            Asset.chevronRight.swiftUIImage
                .foregroundColor(Asset.textAndIconsDisabled.swiftUIColor)
                .padding(.leading, Spacing.small)
            Spacer()
                .frame(width: Spacing.small)
        }
        .frame(height: Heights.selectionBox)
        .containerBackground(CornerRadius.extraSmall)
        .contentShape(Rectangle())
        .onTapGesture {
            viewModel.onNetworkSelectionTap()
        }
    }

    @ViewBuilder
    func derivationPathInput() -> some View {
        HStack(spacing: 0) {
            Spacer()
                .frame(width: Spacing.medium)
            if viewModel.derivationPath == nil {
                Localizable.CreateDerivedKey.Label.Placeholder.path.text
                    .font(PrimaryFont.bodyL.font)
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
            } else if viewModel.derivationPath?.isEmpty == true {
                Localizable.CreateDerivedKey.Label.empyyPath.text
                    .font(PrimaryFont.bodyL.font)
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
            } else {
                Text(viewModel.unwrappedDerivationPath().formattedAsPasswordedPath)
                    .font(PrimaryFont.bodyL.font)
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
            }
            Spacer()
            Asset.chevronRight.swiftUIImage
                .foregroundColor(Asset.textAndIconsDisabled.swiftUIColor)
                .padding(.leading, Spacing.small)
            Spacer()
                .frame(width: Spacing.small)
        }
        .frame(height: Heights.selectionBox)
        .containerBackground(CornerRadius.extraSmall)
        .contentShape(Rectangle())
        .onTapGesture {
            viewModel.onDerivationPathTap()
        }
    }
}

extension CreateDerivedKeyView {
    final class ViewModel: ObservableObject {
        private weak var navigation: NavigationCoordinator!
        private weak var appState: AppState!
        private let networkService: GetAllNetworksService
        private let createKeyService: CreateDerivedKeyService
        @Published var seedName: String = ""
        // State presentatation
        @Published var isPresentingInfoModal: Bool = false
        @Published var presentableInfoModal: ErrorBottomModalViewModel = .derivedKeysInfo()
        @Published var isActionDisabled: Bool = true

        @Published var isPresentingNetworkSelection: Bool = false
        @Published var isPresentingDerivationPath: Bool = false
        @Published var isPresentingConfirmation: Bool = false

        @Published var networkSelection: NetworkSelection = .allowedOnAnyNetwork([])
        @Published var derivationPath: String?
        private let cancelBag = CancelBag()

        init(
            networkService: GetAllNetworksService = GetAllNetworksService(),
            createKeyService: CreateDerivedKeyService = CreateDerivedKeyService()
        ) {
            self.networkService = networkService
            self.createKeyService = createKeyService
            subscribeToChanges()
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func use(appState: AppState) {
            self.appState = appState
            seedName = appState.userData.keysData?.root?.address.seedName ?? ""
            networkService.getNetworks {
                if case let .success(networks) = $0 {
                    appState.userData.allNetworks = networks
                    if let network = networks.first {
                        self.networkSelection = .network(network)
                    }
                }
            }
        }

        func onRightNavigationButtonTap() {
            presentableInfoModal = .derivedKeysInfo()
            isPresentingInfoModal = true
        }

        func onDerivationPathQuestionTap() {
            presentableInfoModal = .derivationPathsInfo()
            isPresentingInfoModal = true
        }

        func onNetworkSelectionTap() {
            isPresentingNetworkSelection = true
        }

        func onDerivationPathTap() {
            isPresentingDerivationPath = true
        }

        func onCreateDerivedKeyTap() {
            let completion: (Result<Void, Error>) -> Void = { result in
                switch result {
                case .success:
                    self.isPresentingConfirmation = true
                case let .failure(error):
                    self.presentableInfoModal = .alertError(message: error.localizedDescription)
                    self.isPresentingInfoModal = true
                }
            }
            switch networkSelection {
            case let .network(network):
                createKeyService.createDerivedKey(seedName, unwrappedDerivationPath(), network.key, completion)
            case .allowedOnAnyNetwork:
                createKeyService.createDerivedKeyOnAllNetworks(seedName, unwrappedDerivationPath(), completion)
            }
        }

        private func subscribeToChanges() {
            $derivationPath.sink {
                self.isActionDisabled = $0 == nil
            }.store(in: cancelBag)
        }

        func unwrappedDerivationPath() -> String {
            derivationPath ?? ""
        }
    }
}

#if DEBUG
    struct CreateDerivedKeyView_Previews: PreviewProvider {
        static var previews: some View {
            CreateDerivedKeyView(
                viewModel: .init()
            )
            .environmentObject(NavigationCoordinator())
        }
    }
#endif
