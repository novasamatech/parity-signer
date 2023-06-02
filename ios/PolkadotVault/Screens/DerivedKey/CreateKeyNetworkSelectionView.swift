//
//  CreateKeyNetworkSelectionView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 10/01/2023.
//

import Combine
import SwiftUI

enum NetworkSelection {
    case network(MmNetwork)
    case allowedOnAnyNetwork([MmNetwork])
}

struct CreateKeyNetworkSelectionView: View {
    @StateObject var viewModel: ViewModel
    @Environment(\.presentationMode) var presentationMode

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            // Navigation Bar
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: Localizable.CreateDerivedKey.Label.title.string,
                    leftButtons: [.init(
                        type: .xmark,
                        action: { presentationMode.wrappedValue.dismiss() }
                    )],
                    backgroundColor: Asset.backgroundPrimary.swiftUIColor
                )
            )
            // Content
            ScrollView(showsIndicators: false) {
                VStack(alignment: .leading, spacing: Spacing.extraSmall) {
                    networkSelection()
                    footer()
                }
            }
            .padding(Spacing.extraSmall)
            // Navigation Links
            NavigationLink(
                destination: DerivationPathNameView(
                    viewModel: .init(
                        seedName: viewModel.seedName,
                        networkSelection: $viewModel.networkSelection,
                        onComplete: viewModel.onKeyCreationComplete
                    )
                )
                .navigationBarHidden(true),
                isActive: $viewModel.isPresentingDerivationPath
            ) { EmptyView() }
        }
        .background(Asset.backgroundPrimary.swiftUIColor)
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
        AttributedInfoBoxView(text: AttributedString(Localizable.CreateDerivedKey.Label.Footer.network.string))
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
            allowOnAnyNetwork()
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
            Asset.chevronRight.swiftUIImage
                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                .padding(.trailing, Spacing.extraSmall)
        }
        .contentShape(Rectangle())
        .padding(.horizontal, Spacing.medium)
        .frame(height: Heights.createKeyNetworkItemHeight)
        .onTapGesture {
            viewModel.selectNetwork(network)
        }
    }

    @ViewBuilder
    func allowOnAnyNetwork() -> some View {
        HStack(alignment: .center, spacing: 0) {
            Localizable.CreateDerivedKey.Label.Network.onAny.text
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .font(PrimaryFont.titleS.font)
            Spacer()
            Asset.chevronRight.swiftUIImage
                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                .padding(.trailing, Spacing.extraSmall)
        }
        .contentShape(Rectangle())
        .padding(.horizontal, Spacing.medium)
        .frame(height: Heights.createKeyNetworkItemHeight)
        .onTapGesture {
            viewModel.selectAllNetworks()
        }
    }
}

extension CreateKeyNetworkSelectionView {
    enum OnCompletionAction: Equatable {
        case derivedKeyCreated
    }

    final class ViewModel: ObservableObject {
        private let cancelBag = CancelBag()
        private let networkService: GetAllNetworksService
        private let keyName: String
        private let createKeyService: CreateDerivedKeyService
        let seedName: String
        @Published var isPresentingDerivationPath: Bool = false
        @Published var networks: [MmNetwork] = []
        @Published var networkSelection: NetworkSelection = .allowedOnAnyNetwork([])

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
            networkService: GetAllNetworksService = GetAllNetworksService(),
            createKeyService: CreateDerivedKeyService = CreateDerivedKeyService(),
            onCompletion: @escaping (OnCompletionAction) -> Void
        ) {
            self.seedName = seedName
            self.keyName = keyName
            self.networkService = networkService
            self.createKeyService = createKeyService
            self.onCompletion = onCompletion
            updateNetworks()
            listenToChanges()
        }

        func selectNetwork(_ network: MmNetwork) {
            networkSelection = .network(network)
            isPresentingDerivationPath = true
        }

        func selectAllNetworks() {
            networkSelection = .allowedOnAnyNetwork(networks)
            isPresentingDerivationPath = true
        }

        func onKeyCreationComplete() {
            onCompletion(.derivedKeyCreated)
            dismissRequest.send()
        }

        func onInfoBoxTap() {
            isNetworkTutorialPresented = true
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
            guard let self = self, !isPresented else { return }
            self.createKeyService.resetNavigationState(self.keyName)
            self.updateNetworks()
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
                    onCompletion: { _ in }
                )
            )
        }
    }
#endif
