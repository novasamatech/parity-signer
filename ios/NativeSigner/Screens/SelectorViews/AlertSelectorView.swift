//
//  AlertSelectorView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 05/08/2022.
//

import SwiftUI

struct AlertSelectorView: View {
    @EnvironmentObject private var data: SignerDataModel
    @EnvironmentObject private var connectivityMediator: ConnectivityMediator
    @EnvironmentObject private var navigation: NavigationCoordinator

    var body: some View {
        AlertSelector(
            alertData: navigation.actionResult.alertData,
            isConnectivityOn: connectivityMediator.isConnectivityOn,
            resetAlert: data.resetAlert,
            navigationRequest: { navigationRequest in
                navigation.perform(navigation: navigationRequest)
            }
        )
    }
}
