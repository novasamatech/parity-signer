//
//  ContentView.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import SwiftUI

struct MainScreenContainer: View {
    @EnvironmentObject private var connectivityMediator: ConnectivityMediator
    @EnvironmentObject private var navigation: NavigationCoordinator
    @StateObject var data: SignerDataModel
    @StateObject var onboarding: OnboardingStateMachine

    var body: some View {
        if !data.protected {
            Localizable.pleaseProtectDeviceWithPinOrPassword.text
                .background(Asset.backgroundPrimary.swiftUIColor)
        } else {
            if data.onboardingDone, data.authenticated {
                AuthenticatedScreenContainer()
                    .environmentObject(data)
                    .environmentObject(onboarding)
            } else if data.onboardingDone {
                UnlockDeviceView(viewModel: .init())
                    .environmentObject(data)
            } else {
                onboarding.currentView()
                    .environmentObject(data)
            }
        }
    }
}
