//
//  ConnectivityAlertButton.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 28/12/2022.
//

import SwiftUI

struct ConnectivityAlertButton: View {
    private let action: () -> Void
    private let isConnectivityOn: Bool

    init(
        action: @escaping () -> Void,
        isConnectivityOn: Bool
    ) {
        self.action = action
        self.isConnectivityOn = isConnectivityOn
    }

    var body: some View {
        Button(
            action: action,
            label: {
                ZStack {
                    Circle()
                        .frame(
                            width: Sizes.connectivityAlertDiameter,
                            height: Sizes.connectivityAlertDiameter,
                            alignment: .center
                        )
                        .foregroundColor(.accentRed400)
                    if isConnectivityOn {
                        Image(.connectivityIsOn)
                            .foregroundColor(.accentForegroundText)
                    } else {
                        Image(.connectivityWasOn)
                            .foregroundColor(.accentForegroundText)
                    }
                }
            }
        )
    }
}
