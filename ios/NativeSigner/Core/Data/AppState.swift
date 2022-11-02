//
//  AppState.swift
//  NativeSigner
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
        var keysData: MKeysNew!
        var allNetworks: [Network] = []
        var selectedNetworks: [Network] = []

        init(
            keysData: MKeysNew! = nil,
            allNetworks: [Network] = [],
            selectedNetworks: [Network] = []
        ) {
            self.keysData = keysData
            self.allNetworks = allNetworks
            self.selectedNetworks = selectedNetworks
        }
    }
}

#if DEBUG
    extension AppState {
        static let preview = AppState(
            userData: UserData(
                keysData: PreviewData.mKeyNew
            )
        )
    }
#endif
