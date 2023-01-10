//
//  CreateDerivedKeyView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 10/01/2023.
//

import SwiftUI

struct CreateDerivedKeyView: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var navigation: NavigationCoordinator

    var body: some View {
        VStack(spacing: 0) {
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: Localizable.CreateDerivedKey.Label.title.string,
                    leftButton: .xmark,
                    rightButton: .questionmark,
                    backgroundColor: Asset.backgroundSystem.swiftUIColor
                )
            )
        }
        .background(Asset.backgroundPrimary.swiftUIColor)
        .onAppear {
            viewModel.use(navigation: navigation)
        }
    }
}

extension CreateDerivedKeyView {
    final class ViewModel: ObservableObject {
        private weak var navigation: NavigationCoordinator!
        private let networkService: GetAllNetworksService
        private let createKeyService: CreateDerivedKeyService
        private let seedName: String
        @Published var networks: [MmNetwork] = []

        init(
            seedName: String,
            networkService: GetAllNetworksService = GetAllNetworksService(),
            createKeyService: CreateDerivedKeyService = CreateDerivedKeyService()
        ) {
            self.seedName = seedName
            self.networkService = networkService
            self.createKeyService = createKeyService
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }
    }
}

#if DEBUG
    struct CreateDerivedKeyView_Previews: PreviewProvider {
        static var previews: some View {
            CreateDerivedKeyView(
                viewModel: .init(seedName: "Parity Keys")
            )
            .environmentObject(NavigationCoordinator())
        }
    }
#endif
