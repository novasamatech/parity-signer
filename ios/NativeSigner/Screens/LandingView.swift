//
//  LandingView.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 23.7.2021.
//

import SwiftUI

struct LandingView: View {
    @State private var tacAccept = false
    @State private var ppAccept = false
    @State private var accept = false
    @ObservedObject var data: SignerDataModel
    var body: some View {
        VStack {
            DocumentModal()
            VStack(spacing: 16) {
                Button(
                    action: {
                        tacAccept.toggle()
                    },
                    label: {
                        HStack {
                            (tacAccept ? Image(.checkmark, variant: .square) : Image(.square)).imageScale(.large)
                            Text("I agree to the terms and conditions")
                                .multilineTextAlignment(.leading)
                            Spacer()
                        }
                    }
                )
                Button(
                    action: {
                        ppAccept.toggle()
                    },
                    label: {
                        HStack {
                            (ppAccept ? Image(.checkmark, variant: .square) : Image(.square)).imageScale(.large)
                            Text("I agree to the privacy policy")
                                .multilineTextAlignment(.leading)
                            Spacer()
                        }
                    }
                )
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
                        secondaryButton: .default(Text("Accept"), action: { data.onboard() })
                    )
                })
            }
        }
        .padding()
    }
}

// struct LandingView_Previews: PreviewProvider {
// static var previews: some View {
// LandingView()
// }
// }
