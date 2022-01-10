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
                .background(Color("Bg100"))
            /*
                .onReceive(data.bsDetector.$canaryDead, perform: { canaryDead in
                    if canaryDead {
                        if data.onboardingDone {
                            device_was_online(nil, data.dbName)
                            data.alert = true
                        }
                    } else {
                        
                    }
                })
             */
        }
    }
}
