//
//  AuthenticatedScreenContainer.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 05/08/2022.
//

import SwiftUI

struct AuthenticatedScreenContainer: View {
    @EnvironmentObject private var connectivityMediator: ConnectivityMediator
    @EnvironmentObject private var navigation: NavigationCoordinator
    @EnvironmentObject private var appState: AppState
    @StateObject var viewModel: ViewModel
    @State private var isShowingQRScanner: Bool = false

    var body: some View {
        ZStack {
            if viewModel.selectedTab == .keys {
                KeySetList(viewModel: .init(tabBarViewModel: tabBarViewModel()))
            }
            if viewModel.selectedTab == .settings {
                SettingsView(viewModel: .init(tabBarViewModel: tabBarViewModel()))
            }
        }
        .animation(.default, value: AnimationDuration.standard)
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
        .fullScreenModal(
            isPresented: $navigation.navigationErrorPresentation.isPresented
        ) {
            ErrorBottomModal(
                viewModel: .alertError(message: navigation.navigationErrorPresentation.errorMessage),
                dismissAction: viewModel.onDismissErrorTap(),
                isShowingBottomAlert: $navigation.navigationErrorPresentation.isPresented
            )
            .clearModalBackground()
        }
        .onAppear {
            viewModel.use(appState: appState)
            viewModel.onAppear()
        }
    }

    private func tabBarViewModel() -> TabBarView.ViewModel {
        .init(
            selectedTab: $viewModel.selectedTab,
            onQRCodeTap: viewModel.onQRCodeTap,
            onKeysTap: viewModel.onKeysTap,
            onSettingsTap: viewModel.onSettingsTap
        )
    }
}

extension AuthenticatedScreenContainer {
    final class ViewModel: ObservableObject {
        private weak var appState: AppState!
        private let initialisationService: AppInitialisationService

        @Published var selectedTab: Tab = .keys
        @Published var isShowingQRScanner: Bool = false

        init(
            initialisationService: AppInitialisationService = AppInitialisationService()
        ) {
            self.initialisationService = initialisationService
        }

        func use(appState: AppState) {
            self.appState = appState
        }

        func onAppear() {
            initialisationService.initialiseAppSession()
        }

        func onQRCodeTap() {
            isShowingQRScanner = true
        }

        func onKeysTap() {
            selectedTab = .keys
        }

        func onSettingsTap() {
            selectedTab = .settings
        }

        func onDismissErrorTap() {
            initialisationService.resetNavigationState()
        }

        func onQRScannerDismiss() {
            appState.userData.keyListRequiresUpdate = true
        }
    }
}
