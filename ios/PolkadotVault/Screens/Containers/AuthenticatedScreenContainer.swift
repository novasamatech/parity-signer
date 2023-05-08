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
                    isPresented: $viewModel.isShowingQRScanner,
                    onComplete: $viewModel.onQRScannerDismissalComplete
                )
            )
        }
        .fullScreenModal(
            isPresented: $navigation.genericError.isPresented
        ) {
            ErrorBottomModal(
                viewModel: .alertError(message: navigation.genericError.errorMessage),
                dismissAction: viewModel.onDismissErrorTap(),
                isShowingBottomAlert: $navigation.genericError.isPresented
            )
            .clearModalBackground()
        }
        .onAppear {
            viewModel.use(navigation: navigation)
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
        private weak var navigation: NavigationCoordinator!
        private weak var appState: AppState!
        private let initialisationService: AppInitialisationService

        @Published var selectedTab: Tab = .keys
        @Published var isShowingQRScanner: Bool = false
        /// Informs main view dispatcher whether we should get back to previous tab when dismissing camera view
        /// or navigate to explicit screen
        /// For some flow, i.e. Key Set Recovery, default navigation would not be intended
        ///
        /// Should be reseted after one dismissal when set to `nil`, so tab navigation is treated as default each other
        /// time
        @Published var qrScannerDismissUpdate: (() -> Void)?
        @Published var onQRScannerDismissalComplete: () -> Void = {}

        init(
            navigation: NavigationCoordinator = NavigationCoordinator(),
            initialisationService: AppInitialisationService = AppInitialisationService()
        ) {
            self.navigation = navigation
            self.initialisationService = initialisationService
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
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
            navigation.performFake(navigation: .init(action: .goBack))
        }

        func onQRScannerDismiss() {
            qrScannerDismissUpdate?()
            qrScannerDismissUpdate = nil
            onQRScannerDismissalComplete()
            onQRScannerDismissalComplete = {}
            appState.userData.keyListRequiresUpdate = true
        }
    }
}
