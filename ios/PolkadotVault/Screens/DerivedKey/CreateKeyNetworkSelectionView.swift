//
//  CreateKeyNetworkSelectionView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 10/01/2023.
//

import Combine
import SwiftUI

struct CreateKeyNetworkSelectionView: View {
    @StateObject var viewModel: ViewModel
    @Environment(\.presentationMode) var presentationMode

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            // Navigation Bar
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: .subtitle(
                        title: Localizable.CreateDerivedKey.Label.title.string,
                        subtitle: Localizable.CreateDerivedKey.Label.subtitle.string
                    ),
                    leftButtons: [.init(
                        type: .xmark,
                        action: { presentationMode.wrappedValue.dismiss() }
                    )],
                    backgroundColor: .backgroundPrimary
                )
            )
            .padding(.bottom, Spacing.extraSmall)
            // Content
            Localizable.CreateDerivedKey.Label.header.text
                .foregroundColor(.textAndIconsPrimary)
                .font(PrimaryFont.bodyL.font)
                .padding(.horizontal, Spacing.large)
                .padding(.bottom, Spacing.small)
            ScrollView(showsIndicators: false) {
                VStack(alignment: .leading, spacing: 0) {
                    networkSelection()
                        .padding(Spacing.extraSmall)
                    footer()
                        .padding(Spacing.medium)
                }
            }
            // Navigation Links
            NavigationLink(
                destination: DerivationPathNameView(
                    viewModel: viewModel.derivationPathViewModel()
                )
                .navigationBarHidden(true),
                isActive: $viewModel.isPresentingDerivationPath
            ) { EmptyView() }
        }
        .background(.backgroundPrimary)
        .onReceive(viewModel.dismissViewRequest) { _ in
            presentationMode.wrappedValue.dismiss()
        }
        .fullScreenModal(
            isPresented: $viewModel.isNetworkTutorialPresented,
            onDismiss: viewModel.updateNetworks
        ) {
            NavigationView {
                AddKeySetUpNetworksStepOneView(viewModel: .init())
                    .navigationBarBackButtonHidden(true)
                    .navigationViewStyle(.stack)
            }
        }
    }

    @ViewBuilder
    func footer() -> some View {
        TransparentHelpBox(text: Localizable.CreateDerivedKey.Label.Footer.network.string)
            .onTapGesture {
                viewModel.onInfoBoxTap()
            }
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
        }
        .containerBackground()
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
            Image(.chevronRight)
                .foregroundColor(.textAndIconsTertiary)
                .padding(.trailing, Spacing.extraSmall)
        }
        .contentShape(Rectangle())
        .padding(.horizontal, Spacing.medium)
        .frame(height: Heights.createKeyNetworkItemHeight)
        .onTapGesture {
            viewModel.selectNetwork(network)
        }
    }
}

extension CreateKeyNetworkSelectionView {
    enum OnCompletionAction: Equatable {
        case derivedKeyCreated
    }

    final class ViewModel: ObservableObject {
        private let cancelBag = CancelBag()
        private let networkService: GetManagedNetworksService
        private let keyName: String
        private let createKeyService: CreateDerivedKeyService
        private let keySet: MKeysNew
        let seedName: String
        @Published var isPresentingDerivationPath: Bool = false
        @Published var networks: [MmNetwork] = []
        @Published var networkSelection: MmNetwork!

        // Tutorial
        @Published var isNetworkTutorialPresented: Bool = false

        var dismissViewRequest: AnyPublisher<Void, Never> {
            dismissRequest.eraseToAnyPublisher()
        }

        private let dismissRequest = PassthroughSubject<Void, Never>()
        private let onCompletion: (OnCompletionAction) -> Void

        init(
            seedName: String,
            keyName: String,
            keySet: MKeysNew,
            networkService: GetManagedNetworksService = GetManagedNetworksService(),
            createKeyService: CreateDerivedKeyService = CreateDerivedKeyService(),
            onCompletion: @escaping (OnCompletionAction) -> Void
        ) {
            self.seedName = seedName
            self.keyName = keyName
            self.keySet = keySet
            self.networkService = networkService
            self.createKeyService = createKeyService
            self.onCompletion = onCompletion
            updateNetworks()
            listenToChanges()
        }

        func selectNetwork(_ network: MmNetwork) {
            networkSelection = network
            isPresentingDerivationPath = true
        }

        private func onKeyCreationComplete() {
            onCompletion(.derivedKeyCreated)
            dismissRequest.send()
        }

        func onInfoBoxTap() {
            isNetworkTutorialPresented = true
        }

        func derivationPathViewModel() -> DerivationPathNameView.ViewModel {
            .init(
                seedName: seedName,
                keySet: keySet,
                networkSelection: networkSelection,
                onComplete: onKeyCreationComplete
            )
        }
    }
}

private extension CreateKeyNetworkSelectionView.ViewModel {
    func updateNetworks() {
        networkService.getNetworks {
            if case let .success(networks) = $0 {
                self.networks = networks
            }
        }
    }

    func listenToChanges() {
        $isNetworkTutorialPresented.sink { [weak self] isPresented in
            guard let self, !isPresented else { return }
            createKeyService.resetNavigationState(keyName)
            updateNetworks()
        }
        .store(in: cancelBag)
    }
}

#if DEBUG
    struct CreateDerivedKeyView_Previews: PreviewProvider {
        static var previews: some View {
            CreateKeyNetworkSelectionView(
                viewModel: .init(
                    seedName: "seedName",
                    keyName: "keyName",
                    keySet: .stub,
                    onCompletion: { _ in }
                )
            )
        }
    }
#endif
