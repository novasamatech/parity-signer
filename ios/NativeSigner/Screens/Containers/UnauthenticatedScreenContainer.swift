//
//  UnauthenticatedScreenContainer.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 05/08/2022.
//

import SwiftUI

struct UnauthenticatedScreenContainer: View {
    @ObservedObject var data: SignerDataModel
    private let seedsMediator: SeedsMediating = ServiceLocator.seedsMediator

    var body: some View {
        Button(
            action: { seedsMediator.refreshSeeds() },
            label: {
                BigButton(
                    text: Localizable.unlockApp.key,
                    action: {
                        seedsMediator.refreshSeeds()
                        data.totalRefresh()
                    }
                )
            }
        )
    }
}
