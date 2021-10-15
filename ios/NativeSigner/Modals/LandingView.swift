//
//  LandingView.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 23.7.2021.
//

import SwiftUI

struct LandingView: View {
    @EnvironmentObject var data: SignerDataModel
    @State var tacAccept = false
    var body: some View {
        ZStack {
            ModalBackdrop()
            VStack {
                if (tacAccept) {
                    ScrollView {
                        Text(data.getPP())
                            .foregroundColor(Color("textMainColor"))
                    }
                } else {
                    ScrollView {
                        Text(data.getTaC())
                            .foregroundColor(Color("textMainColor"))
                    }.padding()
                }
                Button(action: {
                    if tacAccept {
                        data.onboard()
                    } else {
                        tacAccept = true
                    }
                }) {
                    Text("Accept")
                        .font(.largeTitle)
                }.padding()
            }
        }
    }
}

/*
 struct LandingView_Previews: PreviewProvider {
 static var previews: some View {
 LandingView()
 }
 }*/
