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
    @Environment(\.presentationMode) var presentationMode

    var body: some View {
        GeometryReader { geo in
            VStack(alignment: .leading, spacing: 0) {
                // Navigation Bar
                NavigationBarView(
                    viewModel: NavigationBarViewModel(
                        leftButtons: [.init(
                            type: .arrow,
                            action: { presentationMode.wrappedValue.dismiss() }
                        )],
                        backgroundColor: Asset.backgroundPrimary.swiftUIColor
                    )
                )
                // Network List
                ScrollView(showsIndicators: false) {
                    VStack(alignment: .leading, spacing: 0) {
                        // Title + Header
                        mainContent()
                        VStack(alignment: .leading, spacing: Spacing.extraSmall) {
                            networkSelection()
                            footer()
                        }
                        PrimaryButton(
                            action: viewModel.onDoneTap,
                            text: Localizable.CreateKeysForNetwork.Action.create.key,
                            style: .primary()
                        )
                        .padding(Spacing.large)
                    }
                }
                .padding(Spacing.extraSmall)
            }
            .frame(
                minWidth: geo.size.width,
                minHeight: geo.size.height
            )
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
            .onTapGesture {
                viewModel.onInfoBoxTap()
            }
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
    enum OnCompletionAction: Equatable {
        case derivedKeysCreated
    }

    final class ViewModel: ObservableObject {
        private let cancelBag = CancelBag()
        private let networkService: GetAllNetworksService
        private let keyName: String
        private let createKeyService: CreateDerivedKeyService
        let seedName: String
        @Published var isPresentingDerivationPath: Bool = false
        @Published var networks: [MmNetwork] = MmNetwork.stubList
        @Published var selectedNetworks: [MmNetwork] = []

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
            onCompletion(.derivedKeysCreated)
            dismissRequest.send()
        }

        func onInfoBoxTap() {
            isNetworkTutorialPresented = true
        }

        func onDoneTap() {}
    }
}

private extension CreateKeysForNetworksView.ViewModel {
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
    struct CreateKeysForNetworksView_Previews: PreviewProvider {
        static var previews: some View {
            CreateKeysForNetworksView(
                viewModel: .init(
                    seedName: "seedName",
                    keyName: "keyName",
                    onCompletion: { _ in }
                )
            )
        }
    }
#endif
