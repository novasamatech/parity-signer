//
//  QRCodeButton.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 07/09/2023.
//

import SwiftUI

struct QRCodeButton: View {
    private enum Constants {
        static let outerDiameter: CGFloat = 72
        static let innerDiameter: CGFloat = 56
        static let outerWidth: CGFloat = 6
    }

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
                ZStack(alignment: .center) {
                    Circle()
                        .strokeBorder(.backgroundSecondaryInversed, lineWidth: Constants.outerWidth)
                        .frame(width: Constants.outerDiameter, height: Constants.outerDiameter)
                    Circle()
                        .frame(width: Constants.innerDiameter, height: Constants.innerDiameter)
                        .foregroundColor(.pink500)
                    Image(.scanIcon)
                }
            }
        )
        .shadow(color: Color.black.opacity(0.26), radius: 12, x: 0, y: 12)
    }
}

#if DEBUG
    struct QRCodeButton_Previews: PreviewProvider {
        static var previews: some View {
            VStack(alignment: .leading, spacing: 10) {
                QRCodeButton(
                    action: {}
                )
            }
            .preferredColorScheme(.dark)
            .previewLayout(.sizeThatFits)
        }
    }
#endif
