//
//  CapsuleButton.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 14/10/2022.
//

import SwiftUI

struct CapsuleButton: View {
    private let action: () -> Void
    private let icon: Image
    private let title: String

    init(
        action: @escaping () -> Void,
        icon: Image,
        title: String
    ) {
        self.action = action
        self.icon = icon
        self.title = title
    }

    var body: some View {
        Button(action: action) {
            HStack(spacing: Spacing.extraSmall) {
                Text(title)
                    .font(Fontstyle.labelM.base)
                icon
            }
            .foregroundColor(Asset.accentForegroundText.swiftUIColor)
        }
        .padding([.leading], Spacing.medium)
        .padding([.trailing], Spacing.small)
        .frame(height: Heights.capsuleButton)
        .background(Asset.accentPink500.swiftUIColor)
        .clipShape(Capsule())
    }
}

struct CapsuleButton_Previews: PreviewProvider {
    static var previews: some View {
        VStack(alignment: .leading, spacing: 10) {
            Spacer()
            CapsuleButton(
                action: {},
                icon: Asset.arrowForward.swiftUIImage,
                title: Localizable.Scanner.Action.sign.string
            )
            Spacer()
        }
        .background(.black)
    }
}
