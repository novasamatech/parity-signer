//
//  IconButton.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 15/02/2024.
//

import SwiftUI

struct IconButtonStyle: ButtonStyle {
    func makeBody(configuration: Self.Configuration) -> some View {
        configuration.label
            .padding(Spacing.medium)
            .foregroundColor(.textAndIconsSecondary)
            .frame(
                height: Heights.iconButton,
                alignment: .center
            )
    }
}

struct IconButton: View {
    private let action: () -> Void
    private let icon: ImageResource

    init(
        action: @escaping () -> Void,
        icon: ImageResource
    ) {
        self.action = action
        self.icon = icon
    }

    var body: some View {
        Button(action: action) {
            HStack {
                Image(icon)
            }
        }
        .buttonStyle(IconButtonStyle())
    }
}

#if DEBUG
    struct IconButton_Previews: PreviewProvider {
        static var previews: some View {
            VStack(alignment: .leading, spacing: 10) {
                IconButton(
                    action: {},
                    icon: .refreshPassphrase
                )
            }
            .padding()
            .preferredColorScheme(.dark)
            .previewLayout(.sizeThatFits)
        }
    }
#endif
