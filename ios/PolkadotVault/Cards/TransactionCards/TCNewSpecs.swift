//
//  TCNewSpecs.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 21.10.2021.
//

import SwiftUI

struct TCAddNewNetwork: View {
    var value: NetworkSpecs
    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            Localizable.Transaction.AddNetwork.Label.header.text
                .font(PrimaryFont.bodyL.font)
                .foregroundColor(.textAndIconsSecondary)
                .padding(.leading, Spacing.medium)
                .padding(.bottom, Spacing.extraSmall)
            VStack(alignment: .leading, spacing: Spacing.small) {
                rowWrapper(Localizable.Transaction.AddNetwork.Label.name.string, value.title)
                rowWrapper(Localizable.Transaction.AddNetwork.Label.basePrefix.string, String(value.base58prefix))
                rowWrapper(Localizable.Transaction.AddNetwork.Label.decimals.string, String(value.decimals))
                rowWrapper(Localizable.Transaction.AddNetwork.Label.unit.string, value.unit)
                VStack(alignment: .leading, spacing: Spacing.extraSmall) {
                    Localizable.Transaction.AddNetwork.Label.genesisHash.text
                        .foregroundColor(.textAndIconsTertiary)
                    Text(value.genesisHash.formattedAsString)
                        .foregroundColor(.textAndIconsPrimary)
                    Divider()
                }
                rowWrapper(Localizable.Transaction.AddNetwork.Label.crypto.string, value.encryption.rawValue)
                rowWrapper(Localizable.Transaction.AddNetwork.Label.spec.string, value.name)
                HStack {
                    Text(Localizable.Transaction.AddNetwork.Label.logo.string)
                        .foregroundColor(.textAndIconsTertiary)
                    Spacer()
                    NetworkLogoIcon(networkName: value.logo)
                }
                Divider()
                rowWrapper(Localizable.Transaction.AddNetwork.Label.path.string, value.pathId, isLast: true)
            }
            .verticalRoundedBackgroundContainer()
        }
    }

    @ViewBuilder
    private func rowWrapper(
        _ key: String,
        _ value: String,
        isLast: Bool = false
    ) -> some View {
        HStack {
            Text(key)
                .foregroundColor(.textAndIconsTertiary)
            Spacer()
            Text(value)
                .foregroundColor(.textAndIconsPrimary)
        }
        if !isLast {
            Divider()
        }
    }
}

#if DEBUG
    struct TCAddNewNetwork_Previews: PreviewProvider {
        static var previews: some View {
            TCAddNewNetwork(
                value: .stub
            )
        }
    }
#endif
