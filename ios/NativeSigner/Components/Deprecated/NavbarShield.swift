//
//  NavbarShield.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 27.7.2021.
//

import Network
import SwiftUI

struct NavbarShield: View {
    let alert: Bool
    @EnvironmentObject private var navigation: NavigationCoordinator
    @EnvironmentObject private var connectivityMediator: ConnectivityMediator
    @State private var isPresentingConnectivityAlert = false
    @State private var isPresentingSecureAlert = false

    private let resetWarningAction: ResetConnectivtyWarningsAction

    init(
        alert: Bool,
        resetWarningAction: ResetConnectivtyWarningsAction
    ) {
        self.alert = alert
        self.resetWarningAction = resetWarningAction
    }

    var body: some View {
        Button(
            action: {
                if connectivityMediator.isConnectivityOn || alert {
                    isPresentingConnectivityAlert.toggle()
                } else {
                    isPresentingSecureAlert.toggle()
                }
            },
            label: {
                if connectivityMediator.isConnectivityOn {
                    Image(.shield, variant: .slash)
                        .imageScale(.large)
                        .foregroundColor(Asset.signalDanger.swiftUIColor)
                } else {
                    if alert {
                        Image(.exclamationmark, variant: .shield)
                            .imageScale(.large)
                            .foregroundColor(Asset.signalWarning.swiftUIColor)
                    } else {
                        Image(.lock, variants: [.shield, .fill])
                            .imageScale(.large)
                            .foregroundColor(Asset.crypto400.swiftUIColor)
                    }
                }
            }
        ).fullScreenCover(
            isPresented: $isPresentingConnectivityAlert
        ) {
            ErrorBottomModal(
                viewModel: connectivityMediator.isConnectivityOn ? .connectivityOn() : .connectivityWasOn(
                    continueAction: resetWarningAction.resetConnectivityWarnings()
                ),
                isShowingBottomAlert: $isPresentingConnectivityAlert
            )
            .clearModalBackground()
        }
        .alert(
            Localizable.signerIsSecure.key,
            isPresented: $isPresentingSecureAlert,
            actions: {
                Button(Localizable.Common.ok.key) {}
            },
            message: { Localizable.pleaseProceed.text }
        )
    }
}

// struct NavbarShield_Previews: PreviewProvider {
// static var previews: some View {
// NavbarShield().previewLayout(.sizeThatFits)
// }
// }
