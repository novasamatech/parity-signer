//
//  NavbarShield.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 27.7.2021.
//

import SwiftUI
import Network

struct NavbarShield: View {
    @EnvironmentObject var data: SignerDataModel
    @State var alert = false
    var body: some View {
        if data.canaryDead {
            Button(action: {
                alert = true
            }) {
                Image(systemName: "shield.slash")
                    .imageScale(.large)
                    .foregroundColor(Color("SignalDanger"))
            }
            .alert(isPresented: $alert, content: {
                Alert(
                    title: Text("Network connected!"),
                    message: Text("Signer detects currently connected network; please enable airplane mode, disconnect all cables and handle security breach according with your security protocol."),
                    dismissButton: .cancel(Text("Ok"))
                )
            })
        } else {
            if data.alert {
                Button(action: {
                    alert = true
                }) {
                    Image(systemName: "exclamationmark.shield")
                        .imageScale(.large)
                        .foregroundColor(Color("SignalWarning"))
                }
                .alert(isPresented: $alert, content: {
                    Alert(
                        title: Text("Network was connected!"),
                        message: Text("Your Signer device has connected to a WiFi, tether or Bluetooth network since your last acknowledgement and should be considered unsafe to use. Please follow your security protocol"),
                        primaryButton: .cancel(Text("Back")),
                        secondaryButton: .default(Text("Acknowledge and reset"), action: {
                            data.resetAlert()
                        })
                    )
                })
            } else {
                Button(action: {
                    alert = true
                }) {
                    Image(systemName: "lock.shield.fill")
                        .imageScale(.large)
                        .foregroundColor(Color("Crypto400"))
                }
                .alert(isPresented: $alert, content: {
                    Alert(
                        title: Text("Signer is secure"),
                        message: Text("Please proceed"),
                        dismissButton: .cancel(Text("Ok"))
                    )
                })
            }
        }
    }
}

/*
 struct NavbarShield_Previews: PreviewProvider {
 static var previews: some View {
 NavbarShield().previewLayout(.sizeThatFits)
 }
 }*/
