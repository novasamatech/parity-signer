//
//  NetworkSelectionSettings.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 20/12/2022.
//

import SwiftUI

struct NetworkSelectionSettings: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var navigation: NavigationCoordinator

    var body: some View {
        VStack(spacing: 0) {
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: Localizable.Settings.Networks.Label.title.string,
                    leftButton: .arrow,
                    rightButton: .empty,
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
        }
        .background(Asset.backgroundPrimary.swiftUIColor)
        .onAppear {
            viewModel.use(navigation: navigation)
        }
    }

    @ViewBuilder
    func item(for network: MmNetwork) -> some View {
        HStack(alignment: .center, spacing: 0) {
            NetworkLogoIcon(logo: network.logo)
                .padding(.trailing, Spacing.small)
            Text(network.title)
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .font(PrimaryFont.labelL.font)
            Spacer()
            Asset.chevronRight.swiftUIImage
                .frame(width: Sizes.rightChevronContainerSize, height: Sizes.rightChevronContainerSize)
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
        private weak var navigation: NavigationCoordinator!
        let networks: [MmNetwork]

        init(
            networks: [MmNetwork]
        ) {
            self.networks = networks
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func onTap(_ network: MmNetwork) {
            navigation.perform(navigation: .init(action: .goForward, details: network.key))
        }

        func onBackTap() {
            navigation.perform(navigation: .init(action: .goBack))
        }

        func onAddTap() {
            navigation.shouldPresentQRScanner = true
            navigation.performFake(navigation: .init(action: .goBack))
            navigation.performFake(navigation: .init(action: .navbarScan))
        }
    }
}

struct NetworkSelectionSettings_Previews: PreviewProvider {
    static var previews: some View {
        NetworkSelectionSettings(
            viewModel: .init(networks: [])
        )
        .environmentObject(NavigationCoordinator())
    }
}
