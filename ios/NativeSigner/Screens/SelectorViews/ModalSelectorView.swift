//
//  ModalSelectorView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 05/08/2022.
//

import SwiftUI

struct ModalSelectorView: View {
    @ObservedObject var data: SignerDataModel
    @ObservedObject var navigation: NavigationCoordinator

    var body: some View {
        ModalSelector(
            modalData: navigation.actionResult.modalData,
            alert: data.alert,
            alertShow: { data.alertShow = true },
            navigationRequest: { navigationRequest in
                navigation.perform(navigation: navigationRequest)
            },
            removeSeed: { seedName in data.removeSeed(seedName: seedName) },
            restoreSeed: { seedName, seedPhrase, createSeedKeys in data.restoreSeed(
                seedName: seedName, seedPhrase: seedPhrase, createRoots: createSeedKeys
            ) },
            createAddress: { path, seedName in data.createAddress(path: path, seedName: seedName) },
            getSeedForBackup: { seedName in data.getSeed(seedName: seedName, backup: true) },
            sign: { seedName, comment in data.sign(seedName: seedName, comment: comment) }
        )
    }
}
