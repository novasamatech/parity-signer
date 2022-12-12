//
//  CapsuleButton.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 14/10/2022.
//

import SwiftUI

struct CapsuleButton: View {
    private let action: () -> Void
    private let icon: Image?
    private let title: String
    @Binding var isDisabled: Bool

    init(
        action: @escaping () -> Void,
        icon: Image? = nil,
        title: String,
        isDisabled: Binding<Bool> = .constant(false)
    ) {
        self.action = action
        self.icon = icon
        self.title = title
        _isDisabled = isDisabled
    }

    var body: some View {
        Button(action: action) {
            HStack(spacing: Spacing.extraSmall) {
                Text(title)
                    .font(PrimaryFont.labelM.font)
                if let icon = icon {
                    icon
                }
            }
            .foregroundColor(
                isDisabled ? Asset.textAndIconsDisabled.swiftUIColor : Asset.accentForegroundText.swiftUIColor
            )
        }
        .padding([.leading], Spacing.medium)
        .padding([.trailing], icon == nil ? Spacing.medium : Spacing.small)
        .frame(height: Heights.capsuleButton)
        .background(isDisabled ? Asset.fill6.swiftUIColor : Asset.accentPink500.swiftUIColor)
        .clipShape(Capsule())
        .disabled(isDisabled)
    }
}

struct CapsuleButton_Previews: PreviewProvider {
    static var previews: some View {
        VStack(alignment: .leading, spacing: Spacing.medium) {
            Spacer()
            CapsuleButton(
                action: {},
                icon: Asset.arrowForward.swiftUIImage,
                title: Localizable.Scanner.Action.sign.string
            )
            CapsuleButton(
                action: {},
                icon: Asset.arrowForward.swiftUIImage,
                title: Localizable.Scanner.Action.sign.string,
                isDisabled: .constant(true)
            )
            CapsuleButton(
                action: {},
                title: Localizable.Scanner.Action.sign.string
            )
            Spacer()
        }
        .background(.black)
    }
}
