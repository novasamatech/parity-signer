//
//  ContentView.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import SwiftUI

struct MainButtonScreen: View {
    var testValue = DevTestObject()
    @State private var onBoard = OnBoardingStruct()
    var body: some View {
        ZStack {
            VStack {
                Spacer()
                NavigationLink(destination: CameraView()) {
                    VStack() {
                        Image(systemName: "qrcode.viewfinder").imageScale(.large)
                        Text("Scanner")
                    }
                }
                Spacer()
                Footer()
            }
            .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
            .navigationTitle("Parity Signer").navigationBarTitleDisplayMode(.inline).toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    NavbarShield()
                }
            }.background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
            if !onBoard.done {
                LandingView(onBoard: $onBoard)
            }
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
