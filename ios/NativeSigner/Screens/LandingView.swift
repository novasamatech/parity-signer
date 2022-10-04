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
    @EnvironmentObject private var data: SignerDataModel
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
                            Localizable.iAgreeToTheTermsAndConditions.text
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
                            Localizable.iAgreeToThePrivacyPolicy.text
                                .multilineTextAlignment(.leading)
                            Spacer()
                        }
                    }
                )
                BigButton(
                    text: Localizable.next.key,
                    action: {
                        accept = true
                    },
                    isDisabled: !(tacAccept && ppAccept)
                )
                .padding(.top, 16.0)
                .alert(isPresented: $accept, content: {
                    Alert(
                        title: Localizable.acceptPrivacyPolicy.text,
                        primaryButton: .default(Localizable.decline.text),
                        secondaryButton: .default(
                            Localizable.accept.text,
                            action: { data.onboard() }
                        )
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
