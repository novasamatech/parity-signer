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
    enum OnCompletionAction: Equatable {
        case derivedKeysCreated
    }

    final class ViewModel: ObservableObject {
        private let cancelBag = CancelBag()
        private let networkService: GetAllNetworksService
        private let createKeyService: CreateDerivedKeyService
        let seedName: String
        @Published var isPresentingDerivationPath: Bool = false
        @Published var networks: [MmNetwork] = MmNetwork.stubList
        @Published var selectedNetworks: [MmNetwork] = []
        @Binding var isPresented: Bool

        init(
            seedName: String,
            networkService: GetAllNetworksService = GetAllNetworksService(),
            createKeyService: CreateDerivedKeyService = CreateDerivedKeyService(),
            isPresented: Binding<Bool>
        ) {
            self.seedName = seedName
            self.networkService = networkService
            self.createKeyService = createKeyService
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
            // Add logic to create keys

            onKeyCreationComplete()
        }
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
}

#if DEBUG
    struct CreateKeysForNetworksView_Previews: PreviewProvider {
        static var previews: some View {
            CreateKeysForNetworksView(
                viewModel: .init(
                    seedName: "seedName",
                    isPresented: .constant(true)
                )
            )
        }
    }
#endif
