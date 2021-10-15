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
    @State var tacAcceptAlert = false
    @State var ppAcceptAlert = false
    var body: some View {
        VStack {
            DocumentModal(document: .toc)
            Button(action: {
                if tacAccept {
                    tacAccept = false
                } else {
                    tacAcceptAlert = true
                }
            }) {
                HStack {
                    Image(systemName: tacAccept ? "checkmark.square" : "square").imageScale(.large)
                    Text("I agree to the terms and conditions")
                    Spacer()
                }
            }
            .alert(isPresented: $tacAcceptAlert, content: {
                Alert(
                    title: Text("Accept terms and conditions?"),
                    message: Text("Do you accept terms and conditions?"),
                    primaryButton: .cancel(),
                    secondaryButton: .default(Text("Accept"), action: {tacAccept = true})
                )
            })
            Button(action: {
                if ppAccept {
                    ppAccept = false
                } else {
                    ppAcceptAlert = true
                }
            }) {
                HStack {
                    Image(systemName: ppAccept ? "checkmark.square" : "square").imageScale(.large)
                    Text("I agree to the privacy policy")
                    Spacer()
                }
            }
            .alert(isPresented: $ppAcceptAlert, content: {
                Alert(
                    title: Text("Accept privacy policy?"),
                    message: Text("Do you accept privacy policy?"),
                    primaryButton: .cancel(),
                    secondaryButton: .default(Text("Accept"), action: {ppAccept = true})
                )
            }).padding(.vertical, 10)
            Button(action: {
                data.onboard()
            }) {
                Text("Next")
                    .font(.largeTitle)
            }.padding().disabled(!(tacAccept && ppAccept))
        }
    }
}

/*
 struct LandingView_Previews: PreviewProvider {
 static var previews: some View {
 LandingView()
 }
 }*/
