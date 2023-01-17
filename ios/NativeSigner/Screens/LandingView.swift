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
    @State var presentInitialConnectivityWarning: Bool = false
    @EnvironmentObject private var data: SignerDataModel
    @EnvironmentObject private var connectivityMediator: ConnectivityMediator

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
                PrimaryButton(
                    action: {
                        if connectivityMediator.isConnectivityOn {
                            presentInitialConnectivityWarning = true
                        } else {
                            accept = true
                        }
                    },
                    text: Localizable.next.key,
                    style: .primary(isDisabled: .constant(!(tacAccept && ppAccept)))
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
        .fullScreenCover(
            isPresented: $presentInitialConnectivityWarning
        ) {
            ErrorBottomModal(
                viewModel: .connectivityOn(),
                isShowingBottomAlert: $presentInitialConnectivityWarning
            )
            .clearModalBackground()
        }
    }
}

// struct LandingView_Previews: PreviewProvider {
// static var previews: some View {
// LandingView()
// }
// }
