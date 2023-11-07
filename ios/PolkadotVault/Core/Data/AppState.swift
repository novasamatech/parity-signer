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

        init(
            allNetworks: [MmNetwork] = [],
            selectedNetworks: [MmNetwork] = []
        ) {
            self.allNetworks = allNetworks
            self.selectedNetworks = selectedNetworks
        }
    }
}

#if DEBUG
    extension AppState {
        static let preview = AppState(
            userData: UserData(
                allNetworks: MmNetwork.stubList
            )
        )
    }
#endif
