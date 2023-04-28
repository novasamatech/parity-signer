//
//  AppState.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 28/10/2022.
//

import Foundation

final class AppState: ObservableObject {
    let userData: UserData

    init(userData: UserData = UserData()) {
        self.userData = userData
    }
}

extension AppState {
    final class UserData {
        var allNetworks: [MmNetwork] = []
        var selectedNetworks: [MmNetwork] = []
        var verifierDetails: MVerifierDetails!

        init(
            allNetworks: [MmNetwork] = [],
            selectedNetworks: [MmNetwork] = [],
            verifierDetails: MVerifierDetails! = nil
        ) {
            self.allNetworks = allNetworks
            self.selectedNetworks = selectedNetworks
            self.verifierDetails = verifierDetails
        }
    }
}

extension AppState {
    static let preview = AppState(
        userData: UserData(
            allNetworks: PreviewData.networks
        )
    )
}
