//
//  ActionableInfoBoxView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 30/12/2022.
//

import SwiftUI

struct ActionableInfoBoxView: View {
    struct Renderable {
        let text: String
    }

    struct Action {
        let name: String
        let action: () -> Void
    }

    let renderable: Renderable
    let action: Action?

    var body: some View {
        VStack(alignment: .leading, spacing: Spacing.medium) {
            HStack {
                Text(renderable.text)
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    .font(PrimaryFont.bodyM.font)
                    .fixedSize(horizontal: false, vertical: true)
                Spacer().frame(maxWidth: Spacing.medium)
                Asset.infoIconBold.swiftUIImage
                    .foregroundColor(Asset.accentPink300.swiftUIColor)
                    .padding(.leading, Spacing.medium)
            }
            if let action {
                Text(action.name)
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    .font(PrimaryFont.labelS.font)
                    .padding(.vertical, Spacing.extraSmall)
                    .padding(.horizontal, Spacing.medium)
                    .background(Asset.fill6.swiftUIColor)
                    .clipShape(Capsule())
                    .onTapGesture { action.action() }
            }
        }

        .padding(Spacing.medium)
        .containerBackground(CornerRadius.small, state: .actionableInfo)
    }
}

#if DEBUG
    struct ActionableInfoBoxView_Previews: PreviewProvider {
        static var previews: some View {
            VStack(spacing: Spacing.medium) {
                ActionableInfoBoxView(
                    renderable: .init(
                        text: Localizable.ImportKeys.Error.Label.keySetMissing.string
                    ),
                    action: .init(
                        name: Localizable.ImportKeys.Error.Action.keySetMissing.string,
                        action: {}
                    )
                )
                ActionableInfoBoxView(
                    renderable: .init(
                        text: Localizable.ImportKeys.Error.Label.networkMissing.string
                    ),
                    action: .init(
                        name: Localizable.ImportKeys.Error.Action.networkMissing.string,
                        action: {}
                    )
                )
                ActionableInfoBoxView(
                    renderable: .init(
                        text: Localizable.ImportKeys.Error.Label.alreadyImported.string
                    ),
                    action: nil
                )
            }
            .preferredColorScheme(.light)
        }
    }
#endif
