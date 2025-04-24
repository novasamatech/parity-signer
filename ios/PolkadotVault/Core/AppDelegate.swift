//
//  AppDelegate.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 27/12/2023.
//

import UIKit

import FirebaseCore

final class AppDelegate: UIResponder, UIApplicationDelegate {
    func application(
        _: UIApplication,
        shouldAllowExtensionPointIdentifier extensionPointIdentifier: UIApplication.ExtensionPointIdentifier
    ) -> Bool {
        FirebaseApp.configure()
        switch extensionPointIdentifier {
        case .keyboard:
            false
        default:
            true
        }
    }
}
