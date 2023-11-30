//
//  NavbarButton.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 26/08/2022.
//

import SwiftUI

struct NavbarButtonStyle: ButtonStyle {
    func makeBody(configuration: Self.Configuration) -> some View {
        configuration.label
            .foregroundColor(.navbarIcon)
            .frame(
                width: Heights.navigationButton,
                height: Heights.navigationButton,
                alignment: .center
            )
    }
}

struct NavbarButton: View {
    private let action: () -> Void
    private let icon: Image
    @State var isDisabled: Bool

    init(
        action: @escaping () -> Void,
        icon: Image,
        isDisabled: Bool = false
    ) {
        self.action = action
        self.icon = icon
        self.isDisabled = isDisabled
    }

    var body: some View {
        Button(action: action) {
            icon
        }
        .buttonStyle(NavbarButtonStyle())
        .disabled(isDisabled)
    }
}

#if DEBUG
    struct NavbarButton_Previews: PreviewProvider {
        static var previews: some View {
            VStack(alignment: .leading, spacing: 10) {
                NavbarButton(
                    action: {},
                    icon: Image(.arrowBack)
                )
                NavbarButton(
                    action: {},
                    icon: Image(.moreDots)
                )
            }
            .preferredColorScheme(.dark)
            .previewLayout(.sizeThatFits)
            VStack(alignment: .leading, spacing: 10) {
                NavbarButton(
                    action: {},
                    icon: Image(.arrowBack)
                )
                NavbarButton(
                    action: {},
                    icon: Image(.moreDots)
                )
            }
            .preferredColorScheme(.light)
            .previewLayout(.sizeThatFits)
        }
    }
#endif
