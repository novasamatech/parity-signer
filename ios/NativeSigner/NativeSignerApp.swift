//
//  NativeSignerApp.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import SwiftUI

@main
struct NativeSignerApp: App {
    @StateObject var connectivityMediator = ConnectivityMediator()
    @StateObject var navigation = NavigationCoordinator()

    var body: some Scene {
        WindowGroup {
            MainScreenContainer(data: SignerDataModel(
                navigation: navigation,
                connectivityMediator: connectivityMediator
            ))
            .font(Fontstyle.body1.base)
            .background(Asset.bg100.swiftUIColor)
            .environmentObject(navigation)
            .environmentObject(connectivityMediator)
        }
    }
}
