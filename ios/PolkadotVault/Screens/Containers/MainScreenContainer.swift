//
//  ContentView.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import SwiftUI

struct MainScreenContainer: View {
    @EnvironmentObject private var connectivityMediator: ConnectivityMediator
    @EnvironmentObject private var navigation: NavigationCoordinator
    @EnvironmentObject private var passwordProtectionStatePublisher: PasswordProtectionStatePublisher
    @StateObject var data: SharedDataModel
    @StateObject var onboarding: OnboardingStateMachine

    var body: some View {
        switch passwordProtectionStatePublisher.isProtected {
        case true:
            if data.onboardingDone {
                if data.authenticated {
                    AuthenticatedScreenContainer()
                        .environmentObject(data)
                        .environmentObject(onboarding)
                } else {
                    UnlockDeviceView(viewModel: .init())
                        .environmentObject(data)
                }
            } else {
                onboarding.currentView()
                    .environmentObject(data)
            }
        case false:
            DevicePincodeRequired(viewModel: .init())
        }
    }
}
