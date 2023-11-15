//
//  CapsuleButton.swift
//  Polkadot Vault
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
                if let icon {
                    icon
                }
            }
            .foregroundColor(
                isDisabled ? .textAndIconsDisabled : .accentForegroundText
            )
        }
        .padding([.leading], Spacing.medium)
        .padding([.trailing], icon == nil ? Spacing.medium : Spacing.small)
        .frame(height: Heights.capsuleButton)
        .background(isDisabled ? .fill6 : .accentPink500)
        .clipShape(Capsule())
        .disabled(isDisabled)
    }
}

#if DEBUG
    struct CapsuleButton_Previews: PreviewProvider {
        static var previews: some View {
            VStack(alignment: .leading, spacing: Spacing.medium) {
                Spacer()
                CapsuleButton(
                    action: {},
                    icon: Image(.arrowForward),
                    title: Localizable.Scanner.Action.sign.string
                )
                CapsuleButton(
                    action: {},
                    icon: Image(.arrowForward),
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
#endif
