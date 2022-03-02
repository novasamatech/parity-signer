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
    @State var ppAccept = false
    @State var accept = false
    var body: some View {
        VStack {
            DocumentModal()
            VStack(spacing: 16) {
                Button(action: {
                    tacAccept.toggle()
                }) {
                    HStack {
                        Image(systemName: tacAccept ? "checkmark.square" : "square").imageScale(.large)
                        Text("I agree to the terms and conditions")
                            .multilineTextAlignment(.leading)
                        Spacer()
                    }
                }
                Button(action: {
                    ppAccept.toggle()
                }) {
                    HStack {
                        Image(systemName: ppAccept ? "checkmark.square" : "square").imageScale(.large)
                        Text("I agree to the privacy policy")
                            .multilineTextAlignment(.leading)
                        Spacer()
                    }
                }
                BigButton(
                    text: "Next",
                    action: {
                        accept = true
                    },
                    isDisabled: !(tacAccept && ppAccept)
                )
                .padding(.top, 16.0)
                .alert(isPresented: $accept, content: {
                    Alert(
                        title: Text("Accept privacy policy?"),
                        primaryButton: .default(Text("Decline")),
                        secondaryButton: .default(Text("Accept"), action: {data.onboard()})
                    )
                })
            }
        }
        .padding()
    }
}

/*
 struct LandingView_Previews: PreviewProvider {
 static var previews: some View {
 LandingView()
 }
 }*/
