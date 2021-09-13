//
//  ContentView.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import SwiftUI


struct MainScreenContainer: View {
    //var testValue = DevTestObject()
    @EnvironmentObject var data: SignerDataModel
    var body: some View {
        if data.onboardingDone {
            VStack {
                Header()
                switch (data.signerScreen) {
                case .home :
                    TransactionScreen()
                case .keys :
                    KeyManager()
                case .settings :
                    SettingsScreen()
                default:
                    VStack {
                        Text("Please wait")
                            .foregroundColor(Color("textMainColor"))
                    }
                }
                Spacer()
                if data.transactionState == .none {
                    Footer()
                }
            }
        } else {
            LandingView().background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
        }
    }
}

/*
 struct MainButtonScreen_Previews: PreviewProvider {
 static var previews: some View {
 MainScreenContainer()
 }
 }*/
