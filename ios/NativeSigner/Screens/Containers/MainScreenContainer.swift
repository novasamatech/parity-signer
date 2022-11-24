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
        if data.onboardingDone, data.authenticated {
            AuthenticatedScreenContainer()
                .environmentObject(data)
        } else if data.onboardingDone {
            UnauthenticatedScreenContainer()
                .environmentObject(data)
        } else if data.protected {
            LandingView()
                .environmentObject(data)
        } else if data.protected, connectivityMediator.isConnectivityOn {
            Localizable.Connectivity.detected.text
                .background(Asset.bg000.swiftUIColor)
        } else {
            Localizable.pleaseProtectDeviceWithPinOrPassword.text
                .background(Asset.bg000.swiftUIColor)
        }
    }
}

// struct MainButtonScreen_Previews: PreviewProvider {
// static var previews: some View {
// MainScreenContainer()
// }
// }
