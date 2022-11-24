//
//  HeaderViewContainer.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 05/08/2022.
//

import SwiftUI

struct HeaderViewContainer: View {
    @EnvironmentObject private var data: SignerDataModel
    @EnvironmentObject private var navigation: NavigationCoordinator

    var body: some View {
        Header(
            back: navigation.actionResult.back,
            screenLabel: navigation.actionResult.screenLabel,
            screenNameType: navigation.actionResult.screenNameType,
            rightButton: navigation.actionResult.rightButton,
            alert: data.alert,
            alertShow: { data.alertShow = true }
        )
    }
}
