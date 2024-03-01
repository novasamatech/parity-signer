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
                    .lineLimit(nil)
                    .multilineTextAlignment(.leading)
                    .fixedSize(horizontal: false, vertical: true)
                    .foregroundColor(.textAndIconsPrimary)
                    .font(PrimaryFont.bodyM.font)
                Spacer().frame(maxWidth: Spacing.medium)
                Image(.infoIconBold)
                    .foregroundColor(.accentPink300)
                    .padding(.leading, Spacing.medium)
            }
            if let action {
                Text(action.name)
                    .foregroundColor(.textAndIconsPrimary)
                    .font(PrimaryFont.labelS.font)
                    .padding(.vertical, Spacing.extraSmall)
                    .padding(.horizontal, Spacing.medium)
                    .background(.fill6)
                    .clipShape(Capsule())
                    .onTapGesture { action.action() }
            }
        }
        .padding(Spacing.medium)
        .frame(maxWidth: .infinity)
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
