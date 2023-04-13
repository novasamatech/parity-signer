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
    @StateObject var viewModel: ViewModel
    @StateObject var snackBarPresentation = ServiceLocator.bottomSnackbarPresentation
    @GestureState private var dragOffset = CGSize.zero
    @State private var isShowingQRScanner: Bool = false

    var body: some View {
        viewModel.mainScreenFactory.screen(for: navigation.actionResult.screenData)
            .bottomEdgeOverlay(
                overlayView: CameraView(
                    viewModel: .init(
                        isPresented: $navigation.shouldPresentQRScanner
                    )
                ),
                isPresented: $isShowingQRScanner
            )
            .onReceive(navigation.$shouldPresentQRScanner) { shouldPresent in
                withAnimation {
                    if shouldPresent {
                        // Pretend to go to QR Scanner tab, to be able to display transaction later
                        navigation.performFake(navigation: .init(action: .start))
                        navigation.performFake(navigation: .init(action: .navbarScan))
                        isShowingQRScanner = true
                    } else {
                        if let dismissalUpdate = navigation.qrScannerDismissUpdate {
                            // Override default tab navigation when we want to end on specific screen after camera
                            // dismissal
                            dismissalUpdate()
                            navigation.qrScannerDismissUpdate = nil
                        } else if let action = navigation.selectedTab.action {
                            // "Pretend" to go back to previous tab (as we don't change `selectedTab` when showing QR
                            // screen
                            // now), but do
                            // it for real before dismissing QR code scanner to have dataset to display as we might have
                            // ventured
                            // into transaction details
                            navigation.performFake(navigation: .init(action: .start))
                            navigation.perform(navigation: .init(action: action))
                        }
                        isShowingQRScanner = false
                    }
                }
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
    }
}

extension AuthenticatedScreenContainer {
    final class ViewModel: ObservableObject {
        private let navigation: NavigationCoordinator
        let mainScreenFactory: MainScreensFactory

        init(
            navigation: NavigationCoordinator = NavigationCoordinator(),
            mainScreenFactory: MainScreensFactory = MainScreensFactory()
        ) {
            self.navigation = navigation
            self.mainScreenFactory = mainScreenFactory
            navigation.perform(navigation: .init(action: .start))
        }

        func onDismissErrorTap() {
            navigation.perform(navigation: .init(action: .goBack))
        }
    }
}
