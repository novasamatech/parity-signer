//
//  ChooseNetworkForKeyView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 11/01/2023.
//

import SwiftUI

enum NetworkSelection {
    case network(MmNetwork)
    case allowedOnAnyNetwork([MmNetwork])
}

struct ChooseNetworkForKeyView: View {
    private enum Constants {
        static let maxNetworks = 5
    }

    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var appState: AppState

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: {
                viewModel.cancelAction()
            },
            animateBackground: $viewModel.animateBackground,
            ignoredEdges: .bottom,
            safeAreaInsetsMode: .full,
            content: {
                VStack(spacing: 0) {
                    // Header with X button
                    HStack {
                        Localizable.CreateDerivedKey.Modal.Network.title.text
                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                            .font(PrimaryFont.titleS.font)
                        Spacer()
                        CloseModalButton(action: viewModel.cancelAction)
                    }
                    .padding(.leading, Spacing.large)
                    .padding(.trailing, Spacing.medium)
                    Divider()
                        .padding(Spacing.medium)
                    // List of networks
                    networkSelection()
                        .padding(.bottom, Spacing.small)
                }
            }
        )
        .onAppear {
            viewModel.use(appState: appState)
        }
    }

    @ViewBuilder
    func item(for network: MmNetwork) -> some View {
        HStack(alignment: .center, spacing: 0) {
            NetworkLogoIcon(networkName: network.logo)
                .padding(.trailing, Spacing.small)
            Text(network.title)
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .font(PrimaryFont.titleS.font)
            Spacer()
            if viewModel.isSelected(network) {
                Asset.checkmarkList.swiftUIImage
                    .foregroundColor(Asset.accentPink300.swiftUIColor)
            }
        }
        .contentShape(Rectangle())
        .padding(.leading, Spacing.large)
        .padding(.trailing, Spacing.medium)
        .frame(height: Heights.networkFilterItem)
        .onTapGesture {
            viewModel.selectNetwork(network)
        }
    }

    @ViewBuilder
    func allowOnAnyNetwork() -> some View {
        HStack(alignment: .center, spacing: 0) {
            Localizable.CreateDerivedKey.Modal.Network.onAny.text
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .font(PrimaryFont.titleS.font)
            Spacer()
            if case .allowedOnAnyNetwork = viewModel.networkSelection {
                Asset.checkmarkList.swiftUIImage
                    .foregroundColor(Asset.accentPink300.swiftUIColor)
            }
        }
        .contentShape(Rectangle())
        .padding(.leading, Spacing.large)
        .padding(.trailing, Spacing.medium)
        .frame(height: Heights.networkFilterItem)
        .onTapGesture {
            viewModel.selectAllNetworks()
        }
    }

    @ViewBuilder
    func networkSelection() -> some View {
        if viewModel.networks.count > Constants.maxNetworks {
            ScrollView {
                LazyVStack(spacing: Spacing.extraSmall) {
                    ForEach(
                        viewModel.networks,
                        id: \.key
                    ) {
                        item(for: $0)
                    }
                    Divider()
                        .padding(.horizontal, Spacing.medium)
                    allowOnAnyNetwork()
                }
            }
        } else {
            LazyVStack(spacing: Spacing.extraSmall) {
                ForEach(
                    viewModel.networks,
                    id: \.key
                ) {
                    item(for: $0)
                }
                Divider()
                    .padding(.horizontal, Spacing.medium)
                allowOnAnyNetwork()
            }
        }
    }
}

extension ChooseNetworkForKeyView {
    final class ViewModel: ObservableObject {
        private weak var appState: AppState!
        @Published var animateBackground: Bool = false
        @Published var networks: [MmNetwork] = []
        @Binding var networkSelection: NetworkSelection
        @Binding var isPresented: Bool

        init(
            isPresented: Binding<Bool>,
            networkSelection: Binding<NetworkSelection>
        ) {
            _isPresented = isPresented
            _networkSelection = networkSelection
        }

        func use(appState: AppState) {
            self.appState = appState
            networks = appState.userData.allNetworks
        }

        func cancelAction() {
            animateDismissal()
        }

        func isSelected(_ network: MmNetwork) -> Bool {
            if case let .network(selectedNetwork) = networkSelection {
                return selectedNetwork == network
            }
            return false
        }

        func selectNetwork(_ network: MmNetwork) {
            networkSelection = .network(network)
            animateDismissal()
        }

        func selectAllNetworks() {
            networkSelection = .allowedOnAnyNetwork(networks)
            animateDismissal()
        }

        func animateDismissal() {
            Animations.chainAnimation(
                animateBackground.toggle(),
                // swiftformat:disable all
                delayedAnimationClosure: self.hide()
            )
        }

        private func hide() {
            isPresented = false
        }
    }
}

#if DEBUG
struct ChooseNetworkForKeyView_Previews: PreviewProvider {
    static var previews: some View {
        ChooseNetworkForKeyView(
            viewModel: .init(
                isPresented: .constant(true),
                networkSelection: .constant(.allowedOnAnyNetwork([]))
            )
        )
        .environmentObject(AppState.preview)
    }
}
#endif
