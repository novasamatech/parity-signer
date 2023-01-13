//
//  ModalSelectorView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 05/08/2022.
//

import SwiftUI

struct ModalSelectorView: View {
    @EnvironmentObject private var data: SignerDataModel
    @EnvironmentObject private var navigation: NavigationCoordinator
    private let seedsMediator: SeedsMediating = ServiceLocator.seedsMediator

    var body: some View {
        ModalSelector(
            modalData: navigation.actionResult.modalData,
            alert: data.alert,
            alertShow: { data.alertShow = true },
            navigationRequest: { navigationRequest in
                navigation.perform(navigation: navigationRequest)
            },
            removeSeed: { seedName in seedsMediator.removeSeed(seedName: seedName) },
            restoreSeed: { seedName, seedPhrase in seedsMediator.restoreSeed(
                seedName: seedName, seedPhrase: seedPhrase, navigate: true
            ) },
            createAddress: { path, seedName in data.createAddress(path: path, seedName: seedName) },
            sign: { seedName, comment in data.sign(seedName: seedName, comment: comment) }
        )
    }
}
