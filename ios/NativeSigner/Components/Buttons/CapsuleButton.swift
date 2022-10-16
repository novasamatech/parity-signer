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
    @State var isPressed: Bool

    init(
        action: @escaping () -> Void,
        icon: Image,
        title: String,
        isPressed: Bool = false
    ) {
        self.action = action
        self.icon = icon
        self.title = title
        self.isPressed = isPressed
    }

    var body: some View {
        Button(action: action) {
            HStack(spacing: Spacing.extraSmall) {
                icon
                Text(title)
                    .font(Fontstyle.labelM.base)
            }
            .foregroundColor(isPressed ? Asset.accentPink500.swiftUIColor : Asset.accentForegroundText.swiftUIColor)
        }
        .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
        .padding([.top, .bottom], Spacing.extraSmall)
        .padding([.leading, .trailing], Spacing.medium)
        .background(isPressed ? Asset.accentForegroundText.swiftUIColor : Asset.fill30LightOnly.swiftUIColor)
        .clipShape(Capsule())
    }
}

struct CapsuleButton_Previews: PreviewProvider {
    static var previews: some View {
        VStack(alignment: .leading, spacing: 10) {
            Spacer()
            CapsuleButton(
                action: {},
                icon: Asset.scanMultiple.swiftUIImage,
                title: Localizable.Scanner.Action.multiple.string
            )
            CapsuleButton(
                action: {},
                icon: Asset.scanMultiple.swiftUIImage,
                title: Localizable.Scanner.Action.multiple.string,
                isPressed: true
            )
            Spacer()
        }
        .background(.gray)
    }
}
