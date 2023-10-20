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
                    title: .progress(current: viewModel.step, upTo: viewModel.step),
                    leftButtons: [.init(type: .arrow, action: { mode.wrappedValue.dismiss() })],
                    backgroundColor: .backgroundPrimary
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

            .background(.backgroundPrimary)
            .fullScreenModal(
                isPresented: $viewModel.isPresentingError,
                onDismiss: { viewModel.onErrorDismiss?() }
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
                .foregroundColor(.textAndIconsPrimary)
                .font(PrimaryFont.titleL.font)
                .padding(.top, Spacing.extraSmall)
            Text(viewModel.header())
                .foregroundColor(.textAndIconsPrimary)
                .font(PrimaryFont.bodyL.font)
                .padding(.vertical, Spacing.extraSmall)
        }
        .padding(.horizontal, Spacing.large)
        .padding(.bottom, Spacing.medium)
    }

    @ViewBuilder
    func footer() -> some View {
        TransparentHelpBox(text: Localizable.CreateKeysForNetwork.Label.footer.string)
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
                .foregroundColor(.textAndIconsPrimary)
                .font(PrimaryFont.titleS.font)
            Spacer()
            if viewModel.isSelected(network) {
                Image(.checkmarkChecked)
                    .foregroundColor(.accentPink300)
            } else {
                Image(.checkmarkUnchecked)
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
                .foregroundColor(.textAndIconsPrimary)
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
    enum OnCompletionAction: Equatable {
        case createKeySet(seedName: String)
        case recoveredKeySet(seedName: String)
    }

    enum Mode: Equatable {
        case createKeySet
        case recoverKeySet
        case bananaSplit
    }

    final class ViewModel: ObservableObject {
        private enum Constants {
            static let preselectedNetworks: [String] = ["polkadot", "kusama", "westend"]
        }

        private let cancelBag = CancelBag()
        private let networkService: GetManagedNetworksService
        private let createKeySetService: CreateKeySetService
        private let createKeyService: CreateDerivedKeyService
        private let recoveryKeySetService: RecoverKeySetService
        private let seedsMediator: SeedsMediating
        private let seedName: String
        private let seedPhrase: String
        private let mode: Mode
        private let onCompletion: (OnCompletionAction) -> Void
        var onErrorDismiss: (() -> Void)?

        @Binding var isPresented: Bool

        @Published var isPresentingDerivationPath: Bool = false
        @Published var networks: [MmNetwork] = []
        @Published var selectedNetworks: [MmNetwork] = []
        // Error presentatation
        @Published var isPresentingError: Bool = false
        @Published var errorViewModel: ErrorBottomModalViewModel!
        // Confirmation presentation
        @Published var isPresentingConfirmation: Bool = false

        var step: Int {
            switch mode {
            case .bananaSplit:
                2
            case .createKeySet:
                3
            case .recoverKeySet:
                3
            }
        }

        init(
            seedName: String,
            seedPhrase: String,
            mode: Mode,
            networkService: GetManagedNetworksService = GetManagedNetworksService(),
            createKeyService: CreateDerivedKeyService = CreateDerivedKeyService(),
            createKeySetService: CreateKeySetService = CreateKeySetService(),
            recoveryKeySetService: RecoverKeySetService = RecoverKeySetService(),
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
            isPresented: Binding<Bool>,
            onCompletion: @escaping (OnCompletionAction) -> Void
        ) {
            self.seedName = seedName
            self.seedPhrase = seedPhrase
            self.mode = mode
            self.networkService = networkService
            self.createKeyService = createKeyService
            self.createKeySetService = createKeySetService
            self.recoveryKeySetService = recoveryKeySetService
            self.seedsMediator = seedsMediator
            self.onCompletion = onCompletion
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
                continueKeySetAction()
            }
        }

        func onCreateEmptyKeySetTap() {
            continueKeySetAction()
        }
    }
}

extension CreateKeysForNetworksView.ViewModel {
    func title() -> String {
        switch mode {
        case .recoverKeySet:
            Localizable.CreateKeysForNetwork.Label.Title.recover.string
        case .createKeySet,
             .bananaSplit:
            Localizable.CreateKeysForNetwork.Label.Title.create.string
        }
    }

    func header() -> String {
        switch mode {
        case .recoverKeySet:
            Localizable.CreateKeysForNetwork.Label.Header.recover.string
        case .createKeySet,
             .bananaSplit:
            Localizable.CreateKeysForNetwork.Label.Header.create.string
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

    func continueKeySetAction() {
        if seedsMediator.checkSeedPhraseCollision(seedPhrase: seedPhrase) {
            errorViewModel = .seedPhraseAlreadyExists()
            isPresentingError = true
            return
        }
        switch mode {
        case .createKeySet:
            createKeySet(seedPhrase, onComplete: .createKeySet(seedName: seedName))
        case .recoverKeySet,
             .bananaSplit:
            createKeySet(seedPhrase, onComplete: .recoveredKeySet(seedName: seedName))
        }
    }

    func createKeySet(_ seedPhrase: String, onComplete: CreateKeysForNetworksView.OnCompletionAction) {
        seedsMediator.createSeed(
            seedName: seedName,
            seedPhrase: seedPhrase,
            shouldCheckForCollision: false
        )
        createKeySetService.confirmKeySetCreation(
            seedName: seedName,
            seedPhrase: seedPhrase,
            networks: selectedNetworks
        ) { result in
            switch result {
            case .success:
                self.isPresented = false
                self.onCompletion(onComplete)
            case let .failure(error):
                self.onErrorDismiss = { self.isPresented = false }
                self.errorViewModel = .alertError(message: error.localizedDescription)
                self.isPresentingError = true
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
                    seedPhrase: "seedPhrase",
                    mode: .createKeySet,
                    isPresented: .constant(true),
                    onCompletion: { _ in }
                )
            )
        }
    }
#endif
