//
//  ScreenSelectorView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 05/08/2022.
//

import SwiftUI

struct ScreenSelectorView: View {
    @ObservedObject var data: SignerDataModel
    @ObservedObject var navigation: NavigationCoordinator
    private let seedsMediator: SeedsMediating = ServiceLocator.seedsMediator
    var body: some View {
        ScreenSelector(
            data: data,
            navigation: navigation,
            screenData: navigation.actionResult.screenData,
            alert: data.alert,
            navigationRequest: { navigationRequest in
                navigation.perform(navigation: navigationRequest)
            },
            getSeed: { seedName in seedsMediator.getSeed(seedName: seedName) },
            doJailbreak: data.jailbreak,
            pathCheck: { seed, path, network in
                substratePathCheck(
                    seedName: seed, path: path, network: network, dbname: data.dbName
                )
            },
            createAddress: { path, seedName in data.createAddress(path: path, seedName: seedName) },
            checkSeedCollision: { seedName in seedsMediator.checkSeedCollision(seedName: seedName) },
            restoreSeed: { seedName, seedPhrase, createRoots in seedsMediator.restoreSeed(
                seedName: seedName, seedPhrase: seedPhrase, createRoots: createRoots
            ) },
            sign: { seedName, comment in data.sign(seedName: seedName, comment: comment) },
            doWipe: data.wipe,
            alertShow: { data.alertShow = true },
            increment: { seedName, _ in
                let seedPhrase = seedsMediator.getSeed(seedName: seedName)
                if !seedPhrase.isEmpty {
                    navigation.perform(navigation: .init(
                        action: .increment,
                        details: "1",
                        seedPhrase: seedPhrase
                    ))
                }
            }
        )
    }
}
