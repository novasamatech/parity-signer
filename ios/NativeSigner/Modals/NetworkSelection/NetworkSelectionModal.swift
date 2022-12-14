//
//  NetworkSelectionModal.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 12/12/2022.
//

import SwiftUI

struct NetworkSelectionModal: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var navigation: NavigationCoordinator
    @EnvironmentObject private var appState: AppState

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: {
                viewModel.resetAction()
            },
            animateBackground: $viewModel.animateBackground,
            ignoredEdges: .bottom,
            content: {
                VStack {
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
                        .padding(.top, Spacing.medium)
                    // List of networks
                    LazyVStack {
                        ForEach(
                            viewModel.networks
                                .sorted(by: { $0.order < $1.order }),
                            id: \.order
                        ) {
                            item(for: $0)
                        }
                    }
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
            viewModel.use(navigation: navigation)
            viewModel.loadCurrentSelection()
        }
    }

    @ViewBuilder
    func item(for network: Network) -> some View {
        HStack(alignment: .center, spacing: 0) {
            NetworkLogoIcon(logo: network.logo)
                .padding(.trailing, Spacing.small)
            Text(network.title)
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
            // Currently we don't have app wide support for multiple networks being selected at the same time
            viewModel.switchToNetwork(network)
//            viewModel.toggleSelection(network)
        }
    }
}

extension NetworkSelectionModal {
    final class ViewModel: ObservableObject {
        private weak var appState: AppState!
        private weak var navigation: NavigationCoordinator!
        @Published var animateBackground: Bool = false
        @Published var networks: [Network] = []
        @Published var selectedNetworks: [Network] = []
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

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func loadCurrentSelection() {
            selectedNetworks = appState.userData.selectedNetworks
        }

        func resetAction() {
            selectedNetworks = appState.userData.selectedNetworks
            navigation.performFake(navigation: .init(action: .goBack))
            animateDismissal()
        }

        func doneAction() {
            appState.userData.selectedNetworks = selectedNetworks
            navigation.performFake(navigation: .init(action: .goBack))
            animateDismissal()
        }

        func isSelected(_ network: Network) -> Bool {
            selectedNetworks.contains(network)
        }

        func toggleSelection(_ network: Network) {
            if selectedNetworks.contains(network) {
                selectedNetworks.removeAll { $0 == network }
            } else {
                selectedNetworks.append(network)
            }
        }

        func switchToNetwork(_ network: Network) {
            guard !selectedNetworks.contains(network) else { return }
            selectedNetworks = [network]
            navigation.perform(navigation: .init(action: .changeNetwork, details: network.key))
            animateDismissal()
        }

        func hide() {
            isPresented.toggle()
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
        .environmentObject(AppState.preview)
    }
}
