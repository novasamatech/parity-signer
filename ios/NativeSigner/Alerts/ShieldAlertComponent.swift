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
    let isConnectivityOn: Bool
    let content: ShieldAlert?
    var body: some View {
        ZStack {
            if isConnectivityOn {
                Text("")
                    .alert(
                        Localizable.networkConnected.key,
                        isPresented: $show,
                        actions: {
                            Button(Localizable.Common.ok.key) { navigationRequest(.init(action: .goBack)) }
                        },
                        message: { Localizable.networkConnectedMessage.text }
                    )
            } else {
                if content == .past {
                    Text("")
                        .alert(
                            Localizable.networkWasConnected.key,
                            isPresented: $show,
                            actions: {
                                Button(Localizable.back.key) { navigationRequest(.init(action: .goBack)) }
                                Button(Localizable.acknowledgeAndReset.key) {
                                    resetAlert()
                                }
                            },
                            message: { Localizable.networkWasConnectedMessage.text }
                        )
                } else {
                    Text("")
                        .alert(
                            Localizable.signerIsSecure.key,
                            isPresented: $show,
                            actions: {
                                Button(Localizable.Common.ok.key) { navigationRequest(.init(action: .goBack)) }
                            },
                            message: { Localizable.pleaseProceed.text }
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
