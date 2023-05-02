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
    @Environment(\.presentationMode) var presentationMode

    var body: some View {
        VStack(spacing: 0) {
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: Localizable.Settings.Networks.Label.title.string,
                    leftButtons: [.init(
                        type: .arrow,
                        action: { presentationMode.wrappedValue.dismiss() }
                    )],
                    rightButtons: [.init(type: .empty)],
                    backgroundColor: Asset.backgroundPrimary.swiftUIColor
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
                    viewModel: .init(networkKey: viewModel.selectedDetails)
                )
                .navigationBarHidden(true),
                isActive: $viewModel.isPresentingDetails
            ) { EmptyView() }
        }
        .background(Asset.backgroundPrimary.swiftUIColor)
        .onAppear {
            viewModel.use(navigation: navigation)
        }
        .fullScreenCover(
            isPresented: $viewModel.isShowingQRScanner,
            onDismiss: viewModel.onQRScannerDismiss
        ) {
            CameraView(
                viewModel: .init(
                    isPresented: $viewModel.isShowingQRScanner
                )
            )
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
        private let cancelBag = CancelBag()
        private weak var navigation: NavigationCoordinator!
        private let service: ManageNetworksService
        @Published var networks: [MmNetwork] = []
        @Published var selectedDetails: String!
        @Published var isPresentingDetails = false
        @Published var isShowingQRScanner: Bool = false

        init(
            service: ManageNetworksService = ManageNetworksService()
        ) {
            self.service = service
            updateNetworks()
            onDetailsDismiss()
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func onTap(_ network: MmNetwork) {
            selectedDetails = network.key
            isPresentingDetails = true
        }

        func onAddTap() {
            isShowingQRScanner = true
        }

        func onQRScannerDismiss() {
            updateNetworks()
        }
    }
}

private extension NetworkSelectionSettings.ViewModel {
    func onDetailsDismiss() {
        $isPresentingDetails.sink { [weak self] isPresented in
            guard let self = self, !isPresented else { return }
            self.updateNetworks()
        }.store(in: cancelBag)
    }

    func updateNetworks() {
        networks = service.manageNetworks()
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
