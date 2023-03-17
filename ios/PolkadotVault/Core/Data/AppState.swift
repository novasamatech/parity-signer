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
        var keysData: MKeysNew?
        var allNetworks: [MmNetwork] = []
        var selectedNetworks: [MmNetwork] = []

        init(
            keysData: MKeysNew? = nil,
            allNetworks: [MmNetwork] = [],
            selectedNetworks: [MmNetwork] = []
        ) {
            self.keysData = keysData
            self.allNetworks = allNetworks
            self.selectedNetworks = selectedNetworks
        }
    }
}

extension AppState {
    static let preview = AppState(
        userData: UserData(
            keysData: PreviewData.mKeyNew,
            allNetworks: PreviewData.networks
        )
    )
}
