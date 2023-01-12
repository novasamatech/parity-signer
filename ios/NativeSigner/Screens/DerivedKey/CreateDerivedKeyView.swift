//
//  CreateDerivedKeyView.swift
//  NativeSigner
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
                    leftButton: .xmark,
                    rightButton: .questionmark,
                    backgroundColor: Asset.backgroundSystem.swiftUIColor
                ),
                actionModel: .init(
                    rightBarMenuAction: viewModel.onRightNavigationButtonTap
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
                Localizable.CreateDerivedKey.Label.Footer.path.text
                    .font(PrimaryFont.captionM.font)
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
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
                    selectedNetwork: $viewModel.selectedNetwork
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
            if let network = viewModel.selectedNetwork {
                Localizable.CreateDerivedKey.Label.Network.single.text
                    .font(PrimaryFont.bodyL.font)
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                Spacer()
                HStack(spacing: 0) {
                    NetworkLogoIcon(logo: network.logo, size: Heights.networkLogoInCapsule)
                        .padding(.trailing, Spacing.extraSmall)
                    Text(network.title)
                        .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                        .font(PrimaryFont.bodyM.font)
                        .padding(.trailing, Spacing.extraSmall)
                }
                .padding(Spacing.minimal)
                .background(Asset.fill12.swiftUIColor)
                .clipShape(Capsule())
            } else {
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
            Localizable.CreateDerivedKey.Label.Placeholder.path.text
                .font(PrimaryFont.bodyL.font)
                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
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
        private let seedName: String
        // State presentatation
        @Published var isPresentingInfoModal: Bool = false
        @Published var presentableInfoModal: ErrorBottomModalViewModel = .derivedKeysInfo()
        @Published var isActionDisabled: Bool = true

        @Published var isPresentingNetworkSelection: Bool = false
        @Published var isPresentingDerivationPath: Bool = false

        /// If `nil`, switch to `Allowed to use on any network`
        @Published var selectedNetwork: MmNetwork?
        @Published var derivationPath: String?
        private let cancelBag = CancelBag()

        init(
            seedName: String,
            networkService: GetAllNetworksService = GetAllNetworksService(),
            createKeyService: CreateDerivedKeyService = CreateDerivedKeyService()
        ) {
            self.seedName = seedName
            self.networkService = networkService
            self.createKeyService = createKeyService
            subscribeToChanges()
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func use(appState: AppState) {
            self.appState = appState
            networkService.getNetworks {
                if case let .success(networks) = $0 {
                    appState.userData.allNetworks = networks
                    self.selectedNetwork = networks.first
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

        func onDerivationPathTap() {}

        func onCreateDerivedKeyTap() {
            guard let derivationPath = derivationPath else { return }
            let completion: (Result<Void, Error>) -> Void = { result in
                switch result {
                case .success:
                    self.navigation.perform(navigation: .init(action: .goBack))
                case let .failure(error):
                    self.presentableInfoModal = .alertError(message: error.localizedDescription)
                }
            }
            if let selectedNetwork = selectedNetwork {
                createKeyService.createDerivedKey(seedName, derivationPath, selectedNetwork.key, completion)
            } else {
                createKeyService.createDerivedKeyOnAllNetworks(seedName, derivationPath, completion)
            }
        }

        private func subscribeToChanges() {
            $derivationPath.sink {
                self.isActionDisabled = $0 == nil || ($0?.isEmpty == true)
            }.store(in: cancelBag)
        }
    }
}

#if DEBUG
    struct CreateDerivedKeyView_Previews: PreviewProvider {
        static var previews: some View {
            CreateDerivedKeyView(
                viewModel: .init(seedName: "Parity Keys")
            )
            .environmentObject(NavigationCoordinator())
        }
    }
#endif
