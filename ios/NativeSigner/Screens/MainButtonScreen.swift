//
//  ContentView.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import SwiftUI


struct MainButtonScreen: View {
    var testValue = DevTestObject()
    @EnvironmentObject var data: SignerDataModel
    var body: some View {
        if data.onboardingDone {
            ZStack {
                VStack {
                    Spacer()
                    NavigationLink(destination: TransactionScreen()) {
                        VStack() {
                            Image(systemName: "qrcode.viewfinder").imageScale(.large)
                            Image(uiImage: testValue.image!)
                            Text("Scanner")
                        }
                    }
                    Spacer()
            }
            .navigationTitle("Parity Signer").navigationBarTitleDisplayMode(.inline).toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    NavbarShield()
                }
            }.background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
                VStack {
                    Spacer()
                    Footer(caller: "home")
                }
        }
    } else {
        LandingView().background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
        }
    }
}

struct MainButtonScreen_Previews: PreviewProvider {
    static var previews: some View {
        NavigationView {
            MainButtonScreen()
        }
    }
}
