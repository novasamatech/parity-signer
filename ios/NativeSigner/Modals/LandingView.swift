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
            DocumentModal(document: .toc)
            Button(action: {
                tacAccept.toggle()
            }) {
                HStack {
                    Image(systemName: tacAccept ? "checkmark.square" : "square").imageScale(.large)
                    Text("I agree to the terms and conditions")
                    Spacer()
                }
            }
            Button(action: {
                ppAccept.toggle()
            }) {
                HStack {
                    Image(systemName: ppAccept ? "checkmark.square" : "square").imageScale(.large)
                    Text("I agree to the privacy policy")
                    Spacer()
                }
            }
            .padding(.vertical, 10)
            Button(action: {
                accept = true
            }) {
                Text("Next")
                    .font(.largeTitle)
            }
            .padding()
            .disabled(!(tacAccept && ppAccept))
            .alert(isPresented: $accept, content: {
                Alert(
                    title: Text("Accept privacy policy?"),
                    message: Text("Do you accept privacy policy?"),
                    primaryButton: .destructive(Text("Decline")),
                    secondaryButton: .default(Text("Accept"), action: {data.onboard()})
                )
            })
        }
    }
}

/*
 struct LandingView_Previews: PreviewProvider {
 static var previews: some View {
 LandingView()
 }
 }*/
