//
//  UnauthenticatedScreenContainer.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 05/08/2022.
//

import SwiftUI

struct UnauthenticatedScreenContainer: View {
    @EnvironmentObject private var data: SignerDataModel
    private let seedsMediator: SeedsMediating = ServiceLocator.seedsMediator

    var body: some View {
        Button(
            action: { seedsMediator.refreshSeeds() },
            label: {
                PrimaryButton(
                    action: {
                        seedsMediator.refreshSeeds()
                        data.totalRefresh()
                    },
                    text: Localizable.unlockApp.key,
                    style: .primary()
                )
            }
        )
    }
}
