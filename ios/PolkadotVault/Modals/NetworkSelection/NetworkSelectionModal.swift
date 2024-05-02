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
                            .foregroundColor(.textAndIconsPrimary)
                            .font(PrimaryFont.titleS.font)
                        Spacer()
                        CircleButton(action: viewModel.resetAction)
                    }
                    .padding(.leading, Spacing.large)
                    .padding(.trailing, Spacing.medium)
                    Divider()
                        .padding(.vertical, Spacing.medium)
                    // List of networks
                    networkList()

                    // Bottom actions
                    HStack(spacing: Spacing.extraSmall) {
                        ActionButton(
                            action: viewModel.resetAction,
                            text: Localizable.NetworkFilter.Action.reset.key,
                            style: .secondary()
                        )
                        ActionButton(
                            action: viewModel.doneAction,
                            text: Localizable.NetworkFilter.Action.done.key,
                            style: .primary()
                        )
                    }
                    .padding(Spacing.large)
                }
            }
        )
    }

    @ViewBuilder
    func item(for network: MmNetwork) -> some View {
        HStack(alignment: .center, spacing: 0) {
            NetworkLogoIcon(networkName: network.logo)
                .padding(.trailing, Spacing.small)
            Text(network.title.capitalized)
                .foregroundColor(.textAndIconsPrimary)
                .font(PrimaryFont.labelM.font)
            Spacer()
            if viewModel.isSelected(network) {
                Image(.checkmarkChecked)
                    .foregroundColor(.accentPink300)
            } else {
                Image(.checkmarkUnchecked)
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
        @Published var animateBackground: Bool = false
        @Published var networksSelection: [MmNetwork]
        @Binding var networks: [MmNetwork]
        @Binding var selectedNetworks: [MmNetwork]
        @Binding var isPresented: Bool

        init(
            isPresented: Binding<Bool>,
            networks: Binding<[MmNetwork]>,
            selectedNetworks: Binding<[MmNetwork]>
        ) {
            _isPresented = isPresented
            _networks = networks
            _selectedNetworks = selectedNetworks
            _networksSelection = .init(initialValue: selectedNetworks.wrappedValue)
        }

        func cancelAction() {
            animateDismissal()
        }

        func resetAction() {
            selectedNetworks = []
            animateDismissal()
        }

        func doneAction() {
            selectedNetworks = networksSelection
            animateDismissal()
        }

        func isSelected(_ network: MmNetwork) -> Bool {
            networksSelection.contains(network)
        }

        func toggleSelection(_ network: MmNetwork) {
            if networksSelection.contains(network) {
                networksSelection.removeAll { $0 == network }
            } else {
                networksSelection.append(network)
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

#if DEBUG
struct NetworkSelectionModal_Previews: PreviewProvider {
    static var previews: some View {
        NetworkSelectionModal(
            viewModel: .init(
                isPresented: Binding<Bool>.constant(true),
                networks: .constant(MmNetwork.stubList),
                selectedNetworks: .constant([.stub])
            )
        )
    }
}
#endif
