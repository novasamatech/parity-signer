//
// PolkadotVaultApp.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import SwiftUI

@main
struct PolkadotVaultApp: App {
    @StateObject var connectivityMediator = ServiceLocator.connectivityMediator
    @StateObject var navigation = NavigationCoordinator()
    @StateObject var jailbreakDetectionPublisher = JailbreakDetectionPublisher()
    @StateObject var applicationStatePublisher = ApplicationStatePublisher()

    var body: some Scene {
        WindowGroup {
            if jailbreakDetectionPublisher.isJailbroken {
                JailbreakDetectedView()
            } else {
                MainScreenContainer(
                    viewModel: .init(),
                    onboarding: OnboardingStateMachine()
                )
                .font(PrimaryFont.bodyL.font)
                .background(Asset.backgroundPrimary.swiftUIColor)
                .environmentObject(navigation)
                .environmentObject(connectivityMediator)
                .environmentObject(ServiceLocator.appState)
                .environmentObject(jailbreakDetectionPublisher)
                .environmentObject(applicationStatePublisher)
            }
        }
    }
}
