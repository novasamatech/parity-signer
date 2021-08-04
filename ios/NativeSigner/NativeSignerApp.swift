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
            NavigationView {
                MainButtonScreen().navigationBarTitleDisplayMode(.inline).toolbar {
                    ToolbarItem(placement: .navigationBarTrailing) {
                        NavbarShield()
                    }
                }.background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
            }
            .environmentObject(data)
            .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
            
        }
    }
}
