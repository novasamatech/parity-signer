//
//  CircleButton.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 05/09/2022.
//

import SwiftUI

struct CircleButton: View {
    private let image: ImageResource
    private let action: () -> Void

    init(
        image: ImageResource = .xmarkButton,
        action: @escaping () -> Void
    ) {
        self.image = image
        self.action = action
    }

    var body: some View {
        Button(
            action: action,
            label: {
                ZStack {
                    Circle()
                        .frame(width: Sizes.xmarkButtonDiameter, height: Sizes.xmarkButtonDiameter, alignment: .center)
                        .foregroundColor(.fill18)
                    Image(image)
                        .foregroundColor(.textAndIconsPrimary)
                }
            }
        )
    }
}
