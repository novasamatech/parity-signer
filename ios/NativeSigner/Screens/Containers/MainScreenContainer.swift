//
//  ContentView.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import SwiftUI

struct MainScreenContainer: View {
    @StateObject var data: SignerDataModel
    @StateObject var navigation: NavigationCoordinator

    var body: some View {
        if data.onboardingDone, data.authenticated {
            AuthenticatedScreenContainer(data: data, navigation: navigation)
        } else if data.onboardingDone {
            UnauthenticatedScreenContainer(data: data)
        } else if data.protected {
            LandingView(data: data)
        } else if data.protected, data.isConnectivityOn {
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
