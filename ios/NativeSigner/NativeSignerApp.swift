//
//  NativeSignerApp.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import SwiftUI

@main
struct NativeSignerApp: App {
    let navigation = NavigationCoordinator()

    var body: some Scene {
        WindowGroup {
            MainScreenContainer(
                data: SignerDataModel(navigation: navigation),
                navigation: navigation
            )
            .font(Fontstyle.body1.base)
            .background(Asset.bg100.swiftUIColor)
        }
    }
}
