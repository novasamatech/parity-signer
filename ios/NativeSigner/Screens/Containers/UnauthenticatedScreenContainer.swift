//
//  UnauthenticatedScreenContainer.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 05/08/2022.
//

import SwiftUI

struct UnauthenticatedScreenContainer: View {
    @ObservedObject var data: SignerDataModel

    var body: some View {
        Button(
            action: { data.seedsMediator.refreshSeeds() },
            label: {
                BigButton(
                    text: Localizable.unlockApp.key,
                    action: {
                        data.seedsMediator.refreshSeeds()
                        data.totalRefresh()
                    }
                )
            }
        )
    }
}
