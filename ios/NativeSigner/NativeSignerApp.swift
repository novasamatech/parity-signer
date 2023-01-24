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
    @StateObject var appState = AppState()
    @StateObject var jailbreakDetectionPublisher = JailbreakDetectionPublisher()
    @StateObject var applicationStatePublisher = ApplicationStatePublisher()

    var body: some Scene {
        WindowGroup {
            MainScreenContainer(data: SignerDataModel(
                navigation: navigation,
                connectivityMediator: connectivityMediator
            ))
            .onReceive(jailbreakDetectionPublisher.$isJailbroken) { isJailbroken in
                print("Device is Jailbroken: \(isJailbroken)")
                // Perform logic when device is jailbroken
            }
            .font(PrimaryFont.bodyL.font)
            .background(Asset.backgroundPrimary.swiftUIColor)
            .environmentObject(navigation)
            .environmentObject(connectivityMediator)
            .environmentObject(appState)
            .environmentObject(jailbreakDetectionPublisher)
            .environmentObject(applicationStatePublisher)
        }
    }
}
