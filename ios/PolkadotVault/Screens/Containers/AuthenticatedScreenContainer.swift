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
    @StateObject var snackBarPresentation = ServiceLocator.bottomSnackbarPresentation
    @GestureState private var dragOffset = CGSize.zero
    @State private var isShowingQRScanner: Bool = false

    var body: some View {
        viewModel.mainScreenFactory.screen(
            for: navigation.actionResult.screenData,
            onQRCodeTap: viewModel.onQRCodeTap
        )
        .fullScreenCover(
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
        .environmentObject(snackBarPresentation)
        .bottomSnackbar(snackBarPresentation.viewModel, isPresented: $snackBarPresentation.isSnackbarPresented)
        .fullScreenCover(
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
}

extension AuthenticatedScreenContainer {
    final class ViewModel: ObservableObject {
        private weak var navigation: NavigationCoordinator!
        private weak var appState: AppState!
        let mainScreenFactory: MainScreensFactory

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
            mainScreenFactory: MainScreensFactory = MainScreensFactory()
        ) {
            self.navigation = navigation
            self.mainScreenFactory = mainScreenFactory
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func use(appState: AppState) {
            self.appState = appState
        }

        func onAppear() {
            navigation.perform(navigation: .init(action: .start))
        }

        func onQRCodeTap() {
            isShowingQRScanner = true
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
