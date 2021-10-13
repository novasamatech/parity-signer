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
    @GestureState private var dragOffset = CGSize.zero
    var body: some View {
        if data.onboardingDone {
            VStack {
                Header()
                VStack {
                    switch (data.signerScreen) {
                    case .scan :
                        TransactionScreen()
                    case .keys :
                        KeyManager()
                    case .settings :
                        SettingsScreen()
                    case .history :
                        HistoryScreen()
                    }
                    Spacer()
                }
                .gesture(
                    DragGesture().updating($dragOffset, body: { (value, state, transaction) in
                        if value.startLocation.x < 20 && value.translation.width > 100 {
                            data.goBack()
                        }
                    })
                )
                //Certain places are better off without footer
                if (data.transactionState == .none){
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
