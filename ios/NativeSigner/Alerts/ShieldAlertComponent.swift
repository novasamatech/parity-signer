//
//  ShieldAlert.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 9.5.2022.
//

import SwiftUI

struct ShieldAlertComponent: View {
    @State private var show = true
    let resetAlert: () -> Void
    let navigationRequest: NavigationRequest
    let canaryDead: Bool
    let content: ShieldAlert?
    var body: some View {
        ZStack {
            if canaryDead {
                Text("")
                    .alert(
                        "Network connected!",
                        isPresented: $show,
                        actions: {
                            Button("Ok") { navigationRequest(.init(action: .goBack)) }
                        },
                        message: {
                            Text(
                                "Signer detects currently connected network;" +
                                    " please enable airplane mode, disconnect all cables" +
                                    " and handle security breach according with your security protocol."
                            )
                        }
                    )
            } else {
                if content == .past {
                    Text("")
                        .alert(
                            "Network was connected!",
                            isPresented: $show,
                            actions: {
                                Button("Back") { navigationRequest(.init(action: .goBack)) }
                                Button("Acknowledge and reset") {
                                    resetAlert()
                                }
                            },
                            message: {
                                Text(
                                    "Your Signer device has connected to a WiFi," +
                                        " tether or Bluetooth network since your last acknowledgement" +
                                        " and should be considered unsafe to use." +
                                        " Please follow your security protocol"
                                )
                            }
                        )
                } else {
                    Text("")
                        .alert(
                            "Signer is secure",
                            isPresented: $show,
                            actions: {
                                Button("Ok") { navigationRequest(.init(action: .goBack)) }
                            },
                            message: {
                                Text("Please proceed")
                            }
                        )
                }
            }
        }
    }
}

// struct ShieldAlert_Previews: PreviewProvider {
// static var previews: some View {
// ShieldAlert()
// }
// }
