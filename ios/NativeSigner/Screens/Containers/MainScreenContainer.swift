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
    @EnvironmentObject private var passwordProtectionStatePublisher: PasswordProtectionStatePublisher
    @StateObject var data: SignerDataModel

    var body: some View {
        switch passwordProtectionStatePublisher.isProtected {
        case true:
            if data.onboardingDone {
                if data.authenticated {
                    AuthenticatedScreenContainer()
                        .environmentObject(data)
                } else {
                    UnauthenticatedScreenContainer()
                        .environmentObject(data)
                }
            } else {
                OnboardingAgreementsView(viewModel: .init())
                    .environmentObject(data)
            }
        case false:
            Localizable.pleaseProtectDeviceWithPinOrPassword.text
                .background(Asset.backgroundPrimary.swiftUIColor)
        }
    }
}
