//
//  NativeSignerApp.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import SwiftUI

@main
struct NativeSignerApp: App {
    var body: some Scene {
        WindowGroup {
            MainScreenContainer()
                .font(Fontstyle.body1.base)
                .background(Asset.bg100.swiftUIColor)
        }
    }
}
