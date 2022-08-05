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
        } else if data.protected, data.canaryDead {
            Text(
                "Please enable airplane mode, turn off bluetooth and wifi connection" +
                    " and disconnect all cables!"
            ).background(Asset.bg000.swiftUIColor)
        } else {
            Text("Please protect device with pin or password!")
                .background(Asset.bg000.swiftUIColor)
        }
    }
}

// struct MainButtonScreen_Previews: PreviewProvider {
// static var previews: some View {
// MainScreenContainer()
// }
// }
