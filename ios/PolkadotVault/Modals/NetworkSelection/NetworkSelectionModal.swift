//
//  NetworkSelectionModal.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 12/12/2022.
//

import SwiftUI

struct NetworkSelectionModal: View {
    private enum Constants {
        static let maxNetworks = 5
    }

    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var appState: AppState

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: {
                viewModel.resetAction()
            },
            animateBackground: $viewModel.animateBackground,
            ignoredEdges: .bottom,
            content: {
                VStack(spacing: 0) {
                    // Header with X button
                    HStack {
                        Localizable.NetworkFilter.Label.header.text
                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                            .font(PrimaryFont.titleS.font)
                        Spacer()
                        CloseModalButton(action: viewModel.resetAction)
                    }
                    .padding(.leading, Spacing.large)
                    .padding(.trailing, Spacing.medium)
                    Divider()
                        .padding(.vertical, Spacing.medium)
                    // List of networks
                    networkList()

                    // Bottom actions
                    HStack(spacing: Spacing.extraSmall) {
                        SecondaryButton(
                            action: viewModel.resetAction(),
                            text: Localizable.NetworkFilter.Action.reset.key
                        )
                        PrimaryButton(
                            action: viewModel.doneAction,
                            text: Localizable.NetworkFilter.Action.done.key,
                            style: .primary()
                        )
                    }
                    .padding(Spacing.large)
                }
            }
        )
        .onAppear {
            viewModel.use(appState: appState)
            viewModel.loadCurrentSelection()
        }
    }

    @ViewBuilder
    func item(for network: MmNetwork) -> some View {
        HStack(alignment: .center, spacing: 0) {
            NetworkLogoIcon(networkName: network.logo)
                .padding(.trailing, Spacing.small)
            Text(network.title.capitalized)
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .font(PrimaryFont.labelM.font)
            Spacer()
            if viewModel.isSelected(network) {
                Asset.checkmarkChecked.swiftUIImage
            } else {
                Asset.checkmarkUnchecked.swiftUIImage
            }
        }
        .contentShape(Rectangle())
        .padding(.leading, Spacing.large)
        .padding(.trailing, Spacing.medium)
        .frame(height: Heights.networkFilterItem)
        .onTapGesture {
            viewModel.toggleSelection(network)
        }
    }

    @ViewBuilder
    func networkList() -> some View {
        if viewModel.networks.count > Constants.maxNetworks {
            ScrollView {
                LazyVStack {
                    ForEach(
                        viewModel.networks,
                        id: \.key
                    ) {
                        item(for: $0)
                    }
                }
            }
        } else {
            LazyVStack {
                ForEach(
                    viewModel.networks,
                    id: \.key
                ) {
                    item(for: $0)
                }
            }
        }
    }
}

extension NetworkSelectionModal {
    final class ViewModel: ObservableObject {
        private weak var appState: AppState!
        @Published var animateBackground: Bool = false
        @Published var networks: [MmNetwork] = []
        @Published var selectedNetworks: [MmNetwork] = []
        @Binding var isPresented: Bool

        init(
            isPresented: Binding<Bool>
        ) {
            _isPresented = isPresented
        }

        func use(appState: AppState) {
            self.appState = appState
            networks = appState.userData.allNetworks
        }

        func loadCurrentSelection() {
            selectedNetworks = appState.userData.selectedNetworks
        }

        func cancelAction() {
            animateDismissal()
        }

        func resetAction() {
            appState.userData.selectedNetworks = []
            animateDismissal()
        }

        func doneAction() {
            appState.userData.selectedNetworks = selectedNetworks
            animateDismissal()
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

        func hide() {
            isPresented = false
        }

        func animateDismissal() {
            Animations.chainAnimation(
                animateBackground.toggle(),
                // swiftformat:disable all
                delayedAnimationClosure: self.hide()
            )
        }
    }
}

struct NetworkSelectionModal_Previews: PreviewProvider {
    static var previews: some View {
        NetworkSelectionModal(
            viewModel: .init(isPresented: Binding<Bool>.constant(true))
        )
        .environmentObject(NavigationCoordinator())
        .environmentObject(AppState.preview)
    }
}
