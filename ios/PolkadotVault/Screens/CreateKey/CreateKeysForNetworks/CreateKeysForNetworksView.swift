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
    enum OnCompletionAction: Equatable {
        case createKeySet(seedName: String)
        case recoveredKeySet(seedName: String)
        case bananaSplitRecovery(seedName: String)
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
        private let networkService: GetAllNetworksService
        private let createKeySetService: CreateKeySetService
        private let createKeyService: CreateDerivedKeyService
        private let recoveryKeySetService: RecoverKeySetService
        private let bananaSplitRecoveryService: BananaSplitRecoveryService
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
                return 2
            case .createKeySet:
                return 3
            case .recoverKeySet:
                return 3
            }
        }

        init(
            seedName: String,
            seedPhrase: String,
            mode: Mode,
            networkService: GetAllNetworksService = GetAllNetworksService(),
            createKeyService: CreateDerivedKeyService = CreateDerivedKeyService(),
            createKeySetService: CreateKeySetService = CreateKeySetService(),
            recoveryKeySetService: RecoverKeySetService = RecoverKeySetService(),
            bananaSplitRecoveryService: BananaSplitRecoveryService = BananaSplitRecoveryService(),
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
            self.bananaSplitRecoveryService = bananaSplitRecoveryService
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

    func continueKeySetAction() {
        if seedsMediator.checkSeedPhraseCollision(seedPhrase: seedPhrase) {
            errorViewModel = .seedPhraseAlreadyExists()
            isPresentingError = true
            return
        }
        switch mode {
        case .createKeySet:
            createKeySet(seedPhrase)
        case .recoverKeySet:
            recoverKeySet(seedPhrase)
        case .bananaSplit:
            bananaSplitRecovery(seedPhrase)
        }
    }

    func recoverKeySet(_ seedPhrase: String) {
        seedsMediator.createSeed(
            seedName: seedName,
            seedPhrase: seedPhrase,
            shouldCheckForCollision: false
        )
        recoveryKeySetService.finishKeySetRecover(seedPhrase)
        createKeyService.createDerivedKeys(
            seedName,
            seedPhrase,
            networks: selectedNetworks
        ) { result in
            switch result {
            case .success:
                self.isPresented = false
                self.onCompletion(.recoveredKeySet(seedName: self.seedName))
            case let .failure(error):
                self.onErrorDismiss = { self.isPresented = false }
                self.errorViewModel = .alertError(message: error.localizedDescription)
                self.isPresentingError = true
            }
        }
    }

    func createKeySet(_ seedPhrase: String) {
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
                self.onCompletion(.createKeySet(seedName: self.seedName))
            case let .failure(error):
                self.errorViewModel = .alertError(message: error.localizedDescription)
                self.isPresentingError = true
            }
        }
    }

    func bananaSplitRecovery(_ seedPhrase: String) {
        bananaSplitRecoveryService.startBananaSplitRecover(seedName)
        seedsMediator.createSeed(
            seedName: seedName,
            seedPhrase: seedPhrase,
            shouldCheckForCollision: false
        )
        bananaSplitRecoveryService.completeBananaSplitRecovery(seedPhrase)
        createKeyService.createDerivedKeys(
            seedName,
            seedPhrase,
            networks: selectedNetworks
        ) { result in
            switch result {
            case .success:
                self.isPresented = false
                self.onCompletion(.bananaSplitRecovery(seedName: self.seedName))
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
