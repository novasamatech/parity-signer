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
    @StateObject var passwordProtectionStatePublisher = PasswordProtectionStatePublisher()

    var body: some Scene {
        WindowGroup {
            if jailbreakDetectionPublisher.isJailbroken {
                JailbreakDetectedView()
            } else {
                MainScreenContainer(data: SignerDataModel(
                    navigation: navigation,
                    connectivityMediator: connectivityMediator
                ))
                .font(PrimaryFont.bodyL.font)
                .background(Asset.backgroundPrimary.swiftUIColor)
                .environmentObject(navigation)
                .environmentObject(connectivityMediator)
                .environmentObject(appState)
                .environmentObject(jailbreakDetectionPublisher)
                .environmentObject(applicationStatePublisher)
                .environmentObject(passwordProtectionStatePublisher)
            }
        }
    }
}
