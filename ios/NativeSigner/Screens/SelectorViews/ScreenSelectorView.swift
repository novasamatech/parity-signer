//
//  ScreenSelectorView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 05/08/2022.
//

import SwiftUI

struct ScreenSelectorView: View {
    @EnvironmentObject private var data: SignerDataModel
    @EnvironmentObject var navigation: NavigationCoordinator
    private let seedsMediator: SeedsMediating = ServiceLocator.seedsMediator
    private let databaseMediator: DatabaseMediating = DatabaseMediator()

    var body: some View {
        ScreenSelector(
            screenData: navigation.actionResult.screenData,
            navigationRequest: { navigationRequest in
                navigation.perform(navigation: navigationRequest)
            },
            getSeed: { seedName in seedsMediator.getSeed(seedName: seedName) },
            pathCheck: { seed, path, network in
                substratePathCheck(
                    seedName: seed, path: path, network: network, dbname: databaseMediator.databaseName
                )
            },
            createAddress: { path, seedName in data.createAddress(path: path, seedName: seedName) },
            checkSeedCollision: { seedName in seedsMediator.checkSeedCollision(seedName: seedName) },
            restoreSeed: { seedName, seedPhrase, createRoots in seedsMediator.restoreSeed(
                seedName: seedName, seedPhrase: seedPhrase, createRoots: createRoots
            ) },
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
