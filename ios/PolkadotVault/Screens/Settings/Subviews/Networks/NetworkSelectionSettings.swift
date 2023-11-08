//
//  NetworkSelectionSettings.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 20/12/2022.
//

import SwiftUI

struct NetworkSelectionSettings: View {
    @StateObject var viewModel: ViewModel
    @Environment(\.presentationMode) var presentationMode

    var body: some View {
        VStack(spacing: 0) {
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: .title(Localizable.Settings.Networks.Label.title.string),
                    leftButtons: [.init(
                        type: .arrow,
                        action: { presentationMode.wrappedValue.dismiss() }
                    )],
                    rightButtons: [.init(type: .empty)],
                    backgroundColor: .backgroundPrimary
                )
            )
            ScrollView(showsIndicators: false) {
                VStack(alignment: .leading, spacing: 0) {
                    ForEach(viewModel.networks, id: \.key) {
                        item(for: $0)
                    }
                    HStack(alignment: .center, spacing: 0) {
                        Image(.addLarge)
                            .foregroundColor(.textAndIconsTertiary)
                            .frame(width: Heights.networkLogoInCell, height: Heights.networkLogoInCell)
                            .background(Circle().foregroundColor(.fill12))
                            .padding(.trailing, Spacing.small)
                        Text(Localizable.Settings.Networks.Action.add.string)
                            .foregroundColor(.textAndIconsPrimary)
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
                    viewModel: .init(
                        networkKey: viewModel.selectedDetailsKey,
                        networkDetails: viewModel.selectedDetails,
                        onCompletion: viewModel.onNetworkDetailsCompletion(_:)
                    )
                )
                .navigationBarHidden(true),
                isActive: $viewModel.isPresentingDetails
            ) { EmptyView() }
        }
        .background(.backgroundPrimary)
        .fullScreenModal(
            isPresented: $viewModel.isShowingQRScanner,
            onDismiss: viewModel.onQRScannerDismiss
        ) {
            CameraView(
                viewModel: .init(
                    isPresented: $viewModel.isShowingQRScanner
                )
            )
        }
        .bottomSnackbar(
            viewModel.snackbarViewModel,
            isPresented: $viewModel.isSnackbarPresented
        )
        .fullScreenModal(
            isPresented: $viewModel.isPresentingError
        ) {
            ErrorBottomModal(
                viewModel: viewModel.presentableError,
                isShowingBottomAlert: $viewModel.isPresentingError
            )
            .clearModalBackground()
        }
    }

    @ViewBuilder
    func item(for network: MmNetwork) -> some View {
        HStack(alignment: .center, spacing: 0) {
            NetworkLogoIcon(networkName: network.logo)
                .padding(.trailing, Spacing.small)
            Text(network.title.capitalized)
                .foregroundColor(.textAndIconsPrimary)
                .font(PrimaryFont.labelL.font)
            Spacer()
            Image(.chevronRight)
                .frame(width: Sizes.rightChevronContainerSize, height: Sizes.rightChevronContainerSize)
                .foregroundColor(.textAndIconsTertiary)
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
        private let service: GetManagedNetworksService
        private let networkDetailsService: ManageNetworkDetailsService
        @Published var networks: [MmNetwork] = []
        @Published var selectedDetails: MNetworkDetails!
        @Published var selectedDetailsKey: String!
        @Published var isPresentingDetails = false
        @Published var isShowingQRScanner: Bool = false
        var snackbarViewModel: SnackbarViewModel = .init(title: "")
        @Published var isSnackbarPresented: Bool = false
        @Published var isPresentingError: Bool = false
        @Published var presentableError: ErrorBottomModalViewModel = .alertError(message: "")

        init(
            service: GetManagedNetworksService = GetManagedNetworksService(),
            networkDetailsService: ManageNetworkDetailsService = ManageNetworkDetailsService()
        ) {
            self.service = service
            self.networkDetailsService = networkDetailsService
            updateNetworks()
            onDetailsDismiss()
        }

        func onTap(_ network: MmNetwork) {
            selectedDetailsKey = network.key
            networkDetailsService.getNetworkDetails(network.key) { [weak self] result in
                guard let self else { return }
                switch result {
                case let .success(selectedDetails):
                    self.selectedDetails = selectedDetails
                    isPresentingDetails = true
                case let .failure(error):
                    presentableError = .alertError(message: error.localizedDescription)
                    isPresentingError = true
                }
            }
        }

        func onAddTap() {
            isShowingQRScanner = true
        }

        func onQRScannerDismiss() {
            updateNetworks()
        }

        func onNetworkDetailsCompletion(_ completionAction: NetworkSettingsDetails.OnCompletionAction) {
            switch completionAction {
            case let .networkDeleted(networkTitle):
                snackbarViewModel = .init(
                    title: Localizable.Settings.NetworkDetails.DeleteNetwork.Label
                        .confirmation(networkTitle),
                    style: .warning
                )
                isSnackbarPresented = true
            }
        }
    }
}

private extension NetworkSelectionSettings.ViewModel {
    func onDetailsDismiss() {
        $isPresentingDetails.sink { [weak self] isPresented in
            guard let self, !isPresented else { return }
            updateNetworks()
        }.store(in: cancelBag)
    }

    func updateNetworks() {
        service.getNetworks { result in
            switch result {
            case let .success(networks):
                self.networks = networks
            case let .failure(error):
                self.presentableError = .alertError(message: error.localizedDescription)
                self.isPresentingError = true
            }
        }
    }
}

#if DEBUG
    struct NetworkSelectionSettings_Previews: PreviewProvider {
        static var previews: some View {
            NetworkSelectionSettings(
                viewModel: .init()
            )
        }
    }
#endif
