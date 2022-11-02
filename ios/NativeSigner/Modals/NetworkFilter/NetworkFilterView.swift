//
//  NetworkSelectionView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 31/10/2022.
//

import SwiftUI

struct NetworkFilterView: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var navigation: NavigationCoordinator
    @EnvironmentObject private var appState: AppState

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: {
                viewModel.animateDismissal()
            },
            animateBackground: $viewModel.animateBackground,
            ignoredEdges: .bottom,
            content: {
                VStack {
                    // Header with X button
                    HStack {
                        Localizable.NetworkFilter.Label.header.text
                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                            .font(Fontstyle.titleS.base)
                        Spacer()
                        CloseModalButton(action: viewModel.animateDismissal)
                    }
                    .padding(.leading, Spacing.large)
                    .padding(.trailing, Spacing.medium)
                    Divider()
                        .padding(.top, Spacing.medium)
                    // List of networks
                    LazyVStack {
                        ForEach(
                            viewModel.allNetworks
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
            viewModel.set(appState: appState)
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
                .font(Fontstyle.labelM.base)
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
}

struct NetworkFilterView_Previews: PreviewProvider {
    static var previews: some View {
        NetworkFilterView(
            viewModel: .init(allNetworks: PreviewData.networks, isPresented: Binding<Bool>.constant(true))
        )
    }
}
