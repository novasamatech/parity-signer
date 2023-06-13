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
    @Environment(\.presentationMode) var mode: Binding<PresentationMode>
    @Environment(\.safeAreaInsets) private var safeAreaInsets

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            // Navigation Bar
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: .progress(current: 3, upTo: 3),
                    leftButtons: [.init(type: .arrow, action: { mode.wrappedValue.dismiss() })],
                    backgroundColor: Asset.backgroundPrimary.swiftUIColor
                )
            )
            GeometryReader { geo in
                ScrollView(showsIndicators: false) {
                    VStack(alignment: .leading, spacing: 0) {
                        mainContent()
                        networkSelection()
                        footer()
                        Spacer()
                        PrimaryButton(
                            action: viewModel.onDoneTap,
                            text: Localizable.CreateKeysForNetwork.Action.create.key,
                            style: .primary()
                        )
                        .padding(Spacing.large)
                    }
                    .frame(
                        minWidth: geo.size.width,
                        minHeight: geo.size.height
                    )
                }
            }

            .background(Asset.backgroundPrimary.swiftUIColor)
            .fullScreenModal(
                isPresented: $viewModel.isPresentingError
            ) {
                ErrorBottomModal(
                    viewModel: viewModel.errorViewModel,
                    isShowingBottomAlert: $viewModel.isPresentingError
                )
                .clearModalBackground()
            }
            .fullScreenModal(isPresented: $viewModel.isPresentingConfirmation) {
                HorizontalActionsBottomModal(
                    viewModel: .createEmptyKeySet,
                    mainAction: viewModel.onCreateEmptyKeySetTap(),
                    isShowingBottomAlert: $viewModel.isPresentingConfirmation
                )
                .clearModalBackground()
            }
        }
    }

    @ViewBuilder
    func mainContent() -> some View {
        VStack(alignment: .leading, spacing: 0) {
            Text(viewModel.title())
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .font(PrimaryFont.titleL.font)
                .padding(.top, Spacing.extraSmall)
            Text(viewModel.header())
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .font(PrimaryFont.bodyL.font)
                .padding(.vertical, Spacing.extraSmall)
        }
        .padding(.horizontal, Spacing.large)
        .padding(.bottom, Spacing.medium)
    }

    @ViewBuilder
    func footer() -> some View {
        InfoBoxView(text: Localizable.CreateKeysForNetwork.Label.footer.string)
            .padding(.horizontal, Spacing.medium)
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
        .padding(.horizontal, Spacing.extraSmall)
        .padding(.bottom, Spacing.medium)
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
        case bananaSplit
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
        @Binding var isPresented: Bool

        @Published var isPresentingDerivationPath: Bool = false
        @Published var networks: [MmNetwork] = []
        @Published var selectedNetworks: [MmNetwork] = []
        // Error presentatation
        @Published var isPresentingError: Bool = false
        @Published var errorViewModel: ErrorBottomModalViewModel!
        // Confirmation presentation
        @Published var isPresentingConfirmation: Bool = false

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
            if selectedNetworks.isEmpty {
                isPresentingConfirmation = true
            } else {
                createKeySet()
            }
        }

        func onCreateEmptyKeySetTap() {
            createKeySet()
        }
    }
}

extension CreateKeysForNetworksView.ViewModel {
    func title() -> String {
        switch mode {
        case .recoverKeySet:
            return Localizable.CreateKeysForNetwork.Label.Title.recover.string
        case .createKeySet:
            return Localizable.CreateKeysForNetwork.Label.Title.create.string
        case .bananaSplit:
            return Localizable.CreateKeysForNetwork.Label.Title.bananaSplit.string
        }
    }

    func header() -> String {
        switch mode {
        case .recoverKeySet:
            return Localizable.CreateKeysForNetwork.Label.Header.recover.string
        case .createKeySet:
            return Localizable.CreateKeysForNetwork.Label.Header.create.string
        case .bananaSplit:
            return Localizable.CreateKeysForNetwork.Label.Header.bananaSplit.string
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

    func createKeySet() {
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
        case .recoverKeySet,
             .bananaSplit:
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
