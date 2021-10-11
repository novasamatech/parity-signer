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
                case .scan :
                    TransactionScreen()
                case .keys :
                    KeyManager()
                case .settings :
                    SettingsScreen()
                case .history :
                    HistoryScreen()
                default:
                    VStack {
                        Text("Please wait")
                            .foregroundColor(Color("textMainColor"))
                    }
                }
                Spacer()
                //Certain places are better off without footer
                if (data.transactionState == .none && (data.keyManagerModal != .showKey || data.signerScreen != .keys )){
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
