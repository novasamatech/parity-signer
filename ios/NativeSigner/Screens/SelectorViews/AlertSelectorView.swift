//
//  AlertSelectorView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 05/08/2022.
//

import SwiftUI

struct AlertSelectorView: View {
    @ObservedObject var data: SignerDataModel
    @ObservedObject var navigation: NavigationCoordinator

    var body: some View {
        AlertSelector(
            alertData: navigation.actionResult.alertData,
            canaryDead: data.canaryDead,
            resetAlert: data.resetAlert,
            navigationRequest: { navigationRequest in
                navigation.perform(navigation: navigationRequest)
            }
        )
    }
}
