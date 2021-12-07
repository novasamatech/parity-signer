//
//  ContentView.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import SwiftUI


struct MainScreenContainer: View {
    @EnvironmentObject var data: SignerDataModel
    @GestureState private var dragOffset = CGSize.zero
    var body: some View {
        if data.onboardingDone {
            VStack {
                Header()
                ZStack {
                    VStack {
                        ScreenSelector()
                        Spacer()
                    }
                }/*
                  .gesture(
                  DragGesture().updating($dragOffset, body: { (value, state, transaction) in
                  if value.startLocation.x < 20 && value.translation.width > 100 {
                  data.pushButton(buttonID: .GoBack)
                  }
                  })
                  )*/
                //Certain places are better off without footer
                Footer()
                    .padding(.horizontal)
                    .padding(.vertical, 8)
                    .background(Color("backgroundUtility"))
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
