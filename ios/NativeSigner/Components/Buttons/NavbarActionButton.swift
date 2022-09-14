//
//  NavbarActionButton.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 11/09/2022.
//

import SwiftUI

struct NavbarActionButtonStyle: ButtonStyle {
    func makeBody(configuration: Self.Configuration) -> some View {
        configuration.label
            .padding(Spacing.medium)
            .background(Asset.accentPink500.swiftUIColor)
            .foregroundColor(Asset.accentForegroundText.swiftUIColor)
            .font(Fontstyle.labelM.base)
            .frame(
                height: Heights.navigationButton,
                alignment: .center
            )
            .cornerRadius(CornerRadius.large)
    }
}

struct NavbarActionButton: View {
    private let action: () -> Void
    private let title: LocalizedStringKey

    init(
        action: @escaping () -> Void,
        title: LocalizedStringKey
    ) {
        self.action = action
        self.title = title
    }

    var body: some View {
        Button(action: action) {
            HStack {
                Text(title)
            }
        }
        .buttonStyle(NavbarActionButtonStyle())
    }
}

struct NavbarActionButton_Previews: PreviewProvider {
    static var previews: some View {
        VStack(alignment: .leading, spacing: 10) {
            NavbarActionButton(
                action: {},
                title: Localizable.done.key
            )
        }
        .padding()
        .preferredColorScheme(.dark)
        .previewLayout(.sizeThatFits)
    }
}
