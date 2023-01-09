//
//  ConnectivityAlertButton.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 28/12/2022.
//

import SwiftUI

struct ConnectivityAlertButton: View {
    private let action: () -> Void

    init(
        action: @escaping () -> Void
    ) {
        self.action = action
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
                        .foregroundColor(Asset.accentRed400.swiftUIColor)
                    Asset.connectivityShield.swiftUIImage
                        .foregroundColor(Asset.accentForegroundText.swiftUIColor)
                }
            }
        )
    }
}
