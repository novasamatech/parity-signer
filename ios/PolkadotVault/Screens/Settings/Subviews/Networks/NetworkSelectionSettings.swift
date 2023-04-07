//
//  NetworkSelectionSettings.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 20/12/2022.
//

import SwiftUI

struct NetworkSelectionSettings: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var navigation: NavigationCoordinator
    @EnvironmentObject private var appState: AppState
    @Environment(\.presentationMode) var presentationMode

    var body: some View {
        VStack(spacing: 0) {
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: Localizable.Settings.Networks.Label.title.string,
                    leftButtons: [.init(
                        type: .arrow,
                        action: {
                            viewModel.onBackTap()
                            presentationMode.wrappedValue.dismiss()
                        }
                    )],
                    rightButtons: [.init(type: .empty)],
                    backgroundColor: Asset.backgroundSystem.swiftUIColor
                )
            )
            ScrollView(showsIndicators: false) {
                VStack(alignment: .leading, spacing: 0) {
                    ForEach(viewModel.networks, id: \.key) {
                        item(for: $0)
                    }
                    HStack(alignment: .center, spacing: 0) {
                        Asset.add.swiftUIImage
                            .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                            .frame(width: Heights.networkLogoInCell, height: Heights.networkLogoInCell)
                            .background(Circle().foregroundColor(Asset.fill12.swiftUIColor))
                            .padding(.trailing, Spacing.small)
                        Text(Localizable.Settings.Networks.Action.add.string)
                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                            .font(PrimaryFont.labelL.font)
                        Spacer()
                    }
                    .contentShape(Rectangle())
                    .padding(.horizontal, Spacing.medium)
                    .frame(height: Heights.networkSelectionSettings)
                    .onTapGesture {
                        viewModel.onAddTap()
                    }
                }
            }
            NavigationLink(
                destination: NetworkSettingsDetails(
                    viewModel: .init(networkDetails: viewModel.selectedDetails)
                )
                .navigationBarHidden(true),
                isActive: $viewModel.isPresentingDetails
            ) { EmptyView() }
        }
        .background(Asset.backgroundPrimary.swiftUIColor)
        .onAppear {
            viewModel.use(navigation: navigation)
            viewModel.use(appState: appState)
        }
    }

    @ViewBuilder
    func item(for network: MmNetwork) -> some View {
        HStack(alignment: .center, spacing: 0) {
            NetworkLogoIcon(networkName: network.logo)
                .padding(.trailing, Spacing.small)
            Text(network.title.capitalized)
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .font(PrimaryFont.labelL.font)
            Spacer()
            Asset.chevronRight.swiftUIImage
                .frame(width: Sizes.rightChevronContainerSize, height: Sizes.rightChevronContainerSize)
                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
        }
        .contentShape(Rectangle())
        .padding(.horizontal, Spacing.medium)
        .frame(height: Heights.networkSelectionSettings)
        .onTapGesture {
            viewModel.onTap(network)
        }
    }
}

extension NetworkSelectionSettings {
    final class ViewModel: ObservableObject {
        private weak var appState: AppState!
        private weak var navigation: NavigationCoordinator!
        @Published var networks: [MmNetwork] = []
        @Published var selectedDetails: MNetworkDetails!
        @Published var isPresentingDetails = false

        init() {}

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func use(appState: AppState) {
            self.appState = appState
            networks = appState.userData.manageNetworks.networks
        }

        func onTap(_ network: MmNetwork) {
            guard case let .nNetworkDetails(value) = navigation
                .performFake(navigation: .init(action: .goForward, details: network.key)).screenData else { return }
            selectedDetails = value
            isPresentingDetails = true
        }

        func onBackTap() {
            appState.userData.manageNetworks = nil
            navigation.performFake(navigation: .init(action: .goBack))
        }

        func onAddTap() {
            navigation.shouldPresentQRScanner = true
//            navigation.performFake(navigation: .init(action: .goBack))
//            navigation.performFake(navigation: .init(action: .navbarScan))
        }
    }
}

struct NetworkSelectionSettings_Previews: PreviewProvider {
    static var previews: some View {
        NetworkSelectionSettings(
            viewModel: .init()
        )
        .environmentObject(NavigationCoordinator())
    }
}
