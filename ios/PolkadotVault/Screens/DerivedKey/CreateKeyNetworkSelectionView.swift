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
    }

    @ViewBuilder
    func footer() -> some View {
        AttributedInfoBoxView(text: AttributedString(Localizable.CreateDerivedKey.Label.Footer.network.string))
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
    final class ViewModel: ObservableObject {
        private let cancelBag = CancelBag()
        private let networkService: GetAllNetworksService
        @Published var seedName: String = ""
        @Published var isPresentingDerivationPath: Bool = false
        @Published var networks: [MmNetwork] = []
        @Published var networkSelection: NetworkSelection = .allowedOnAnyNetwork([])
        var dismissViewRequest: AnyPublisher<Void, Never> {
            dismissRequest.eraseToAnyPublisher()
        }

        private let dismissRequest = PassthroughSubject<Void, Never>()

        init(
            seedName: String,
            networkService: GetAllNetworksService = GetAllNetworksService()
        ) {
            _seedName = .init(initialValue: seedName)
            self.networkService = networkService
            updateNetworks()
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
            dismissRequest.send()
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
}

#if DEBUG
    struct CreateDerivedKeyView_Previews: PreviewProvider {
        static var previews: some View {
            CreateKeyNetworkSelectionView(
                viewModel: .init(seedName: "seedName")
            )
            .environmentObject(NavigationCoordinator())
        }
    }
#endif
