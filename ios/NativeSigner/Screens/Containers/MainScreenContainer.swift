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

    var body: some View {
        if !data.protected {
            Localizable.pleaseProtectDeviceWithPinOrPassword.text
                .background(Asset.backgroundPrimary.swiftUIColor)
        } else {
            if data.onboardingDone, data.authenticated {
                AuthenticatedScreenContainer()
                    .environmentObject(data)
            } else if data.onboardingDone {
                UnauthenticatedScreenContainer()
                    .environmentObject(data)
            } else {
                OnboardingAgreementsView(viewModel: .init())
                    .environmentObject(data)
            }
        }
    }
}
