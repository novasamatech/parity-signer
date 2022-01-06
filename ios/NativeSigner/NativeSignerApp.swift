//
//  NativeSignerApp.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import SwiftUI

@main
struct NativeSignerApp: App {
    @StateObject var data = SignerDataModel()
    var body: some Scene {
        WindowGroup {
            MainScreenContainer()
                .environmentObject(data)
                .font(FBase(style: .body1))
                .background(Color("Bg100")
            )
        }
    }
}
