//
//  CreateKeysForNetworksView.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 30/05/2023.
//

import Combine
import SwiftUI

struct CreateKeysForNetworksView: View {
    @StateObject var viewModel: ViewModel

    var body: some View {
        GeometryReader { geo in
            VStack(alignment: .leading, spacing: 0) {
                // Navigation Bar
                NavigationBarView(
                    viewModel: NavigationBarViewModel(
                        backgroundColor: Asset.backgroundPrimary.swiftUIColor
                    )
                )
                ScrollView(showsIndicators: false) {
                    VStack(alignment: .leading, spacing: 0) {
                        // Title + Header
                        mainContent()
                            .padding(.bottom, Spacing.medium)
                        // Network List
                        networkSelection()
                            .padding(.bottom, Spacing.medium)
                        footer()
                    }
                }
                .padding(Spacing.extraSmall)
                Spacer()
                PrimaryButton(
                    action: viewModel.onDoneTap,
                    text: Localizable.CreateKeysForNetwork.Action.create.key,
                    style: .primary()
                )
                .padding(Spacing.large)
            }
            .background(Asset.backgroundPrimary.swiftUIColor)
            .frame(
                minWidth: geo.size.width,
                minHeight: geo.size.height
            )
            .fullScreenModal(
                isPresented: $viewModel.isPresentingError
            ) {
                ErrorBottomModal(
                    viewModel: viewModel.errorViewModel,
                    isShowingBottomAlert: $viewModel.isPresentingError
                )
                .clearModalBackground()
            }
        }
    }

    @ViewBuilder
    func mainContent() -> some View {
        VStack(alignment: .leading, spacing: 0) {
            Localizable.CreateKeysForNetwork.Label.title.text
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .font(PrimaryFont.titleL.font)
                .padding(.top, Spacing.extraSmall)
            Localizable.CreateKeysForNetwork.Label.header.text
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .font(PrimaryFont.bodyL.font)
                .padding(.vertical, Spacing.extraSmall)
        }
        .padding(.horizontal, Spacing.large)
    }

    @ViewBuilder
    func footer() -> some View {
        InfoBoxView(text: Localizable.CreateKeysForNetwork.Label.footer.string)
            .padding(.bottom, Spacing.large)
    }

    @ViewBuilder
    func networkSelection() -> some View {
        LazyVStack(spacing: 0) {
            ForEach(
                viewModel.networks,
                id: \.key
            ) {
                item(for: $0)
                Divider()
                    .padding(.horizontal, Spacing.medium)
            }
            selectAllNetworks()
        }
        .containerBackground()
    }

    @ViewBuilder
    func item(for network: MmNetwork) -> some View {
        HStack(alignment: .center, spacing: 0) {
            NetworkLogoIcon(networkName: network.logo)
                .padding(.trailing, Spacing.small)
            Text(network.title.capitalized)
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .font(PrimaryFont.titleS.font)
            Spacer()
            if viewModel.isSelected(network) {
                Asset.checkmarkChecked.swiftUIImage
            } else {
                Asset.checkmarkUnchecked.swiftUIImage
            }
        }
        .contentShape(Rectangle())
        .padding(.horizontal, Spacing.medium)
        .frame(height: Heights.createKeysForNetworkItemHeight)
        .onTapGesture {
            viewModel.toggleSelection(network)
        }
    }

    @ViewBuilder
    func selectAllNetworks() -> some View {
        HStack(alignment: .center, spacing: 0) {
            Localizable.CreateKeysForNetwork.Action.selectAll.text
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .font(PrimaryFont.titleS.font)
            Spacer()
        }
        .contentShape(Rectangle())
        .padding(.horizontal, Spacing.medium)
        .frame(height: Heights.createKeysForNetworkItemHeight)
        .onTapGesture {
            viewModel.selectAllNetworks()
        }
    }
}

extension CreateKeysForNetworksView {
    enum Mode: Equatable {
        case createKeySet(seedPhrase: String)
        case recoverKeySet
    }

    enum OnCompletionAction: Equatable {
        case derivedKeysCreated
    }

    final class ViewModel: ObservableObject {
        private enum Constants {
            static let preselectedNetworks: [String] = ["polkadot", "kusama", "westend"]
        }

        private let cancelBag = CancelBag()
        private let networkService: GetAllNetworksService
        private let createKeySetService: CreateKeySetService
        private let createKeyService: CreateDerivedKeyService
        private let seedsMediator: SeedsMediating
        private let seedName: String
        private let mode: Mode
        @Published var isPresentingDerivationPath: Bool = false
        @Published var networks: [MmNetwork] = []
        @Published var selectedNetworks: [MmNetwork] = []
        // Error presentatation
        @Published var isPresentingError: Bool = false
        @Published var errorViewModel: ErrorBottomModalViewModel!

        @Binding var isPresented: Bool

        init(
            seedName: String,
            mode: Mode,
            networkService: GetAllNetworksService = GetAllNetworksService(),
            createKeyService: CreateDerivedKeyService = CreateDerivedKeyService(),
            createKeySetService: CreateKeySetService = CreateKeySetService(),
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
            isPresented: Binding<Bool>
        ) {
            self.seedName = seedName
            self.mode = mode
            self.networkService = networkService
            self.createKeyService = createKeyService
            self.createKeySetService = createKeySetService
            self.seedsMediator = seedsMediator
            _isPresented = isPresented
            updateNetworks()
        }

        func selectAllNetworks() {
            selectedNetworks = networks
        }

        func isSelected(_ network: MmNetwork) -> Bool {
            selectedNetworks.contains(network)
        }

        func toggleSelection(_ network: MmNetwork) {
            if selectedNetworks.contains(network) {
                selectedNetworks.removeAll { $0 == network }
            } else {
                selectedNetworks.append(network)
            }
        }

        func onKeyCreationComplete() {
            isPresented = false
        }

        func onDoneTap() {
            switch mode {
            case let .createKeySet(seedPhrase):
                seedsMediator.createSeed(
                    seedName: seedName,
                    seedPhrase: seedPhrase,
                    shouldCheckForCollision: true
                )
                createKeySetService.confirmKeySetCreation(
                    seedName: seedName,
                    seedPhrase: seedPhrase,
                    networks: selectedNetworks
                ) { result in
                    switch result {
                    case .success:
                        self.isPresented = false
                    case let .failure(error):
                        self.errorViewModel = .alertError(message: error.localizedDescription)
                        self.isPresentingError = true
                    }
                }
            case .recoverKeySet:
                createKeyService.createDerivedKeys(
                    seedName,
                    networks: selectedNetworks
                ) { result in
                    switch result {
                    case .success:
                        self.isPresented = false
                    case let .failure(error):
                        self.errorViewModel = .alertError(message: error.localizedDescription)
                        self.isPresentingError = true
                    }
                }
            }
        }
    }
}

private extension CreateKeysForNetworksView.ViewModel {
    func updateNetworks() {
        networkService.getNetworks {
            guard case let .success(networks) = $0 else { return }
            self.networks = networks
            self.selectedNetworks = networks.filter { Constants.preselectedNetworks.contains($0.title) }
        }
    }
}

#if DEBUG
    struct CreateKeysForNetworksView_Previews: PreviewProvider {
        static var previews: some View {
            CreateKeysForNetworksView(
                viewModel: .init(
                    seedName: "seedName",
                    mode: .createKeySet(seedPhrase: ""),
                    isPresented: .constant(true)
                )
            )
        }
    }
#endif
