//
//  TCAddNewNetwork.swift
//  NativeSigner
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
                .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                .padding(.leading, Spacing.medium)
                .padding(.bottom, Spacing.extraSmall)
            VStack(alignment: .leading, spacing: Spacing.small) {
                rowWrapper(Localizable.Transaction.AddNetwork.Label.name.string, value.title)
                rowWrapper(Localizable.Transaction.AddNetwork.Label.basePrefix.string, String(value.base58prefix))
                rowWrapper(Localizable.Transaction.AddNetwork.Label.decimals.string, String(value.decimals))
                rowWrapper(Localizable.Transaction.AddNetwork.Label.unit.string, value.unit)
                VStack(alignment: .leading, spacing: Spacing.extraSmall) {
                    Localizable.Transaction.AddNetwork.Label.genesisHash.text
                        .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    Text(value.genesisHash.formattedAsString)
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    Divider()
                }
                rowWrapper(Localizable.Transaction.AddNetwork.Label.crypto.string, value.encryption.rawValue)
                rowWrapper(Localizable.Transaction.AddNetwork.Label.spec.string, value.name)
                HStack {
                    Text(Localizable.Transaction.AddNetwork.Label.logo.string)
                        .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
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
                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
            Spacer()
            Text(value)
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
        }
        if !isLast {
            Divider()
        }
    }
}

struct TCAddNewNetwork_Previews: PreviewProvider {
    static var previews: some View {
        TCAddNewNetwork(
            value: NetworkSpecs(
                base58prefix: 231,
                color: "black",
                decimals: 4,
                encryption: .sr25519,
                genesisHash: H256(repeating: 3, count: 4),
                logo: "polkadot",
                name: "polkadot",
                pathId: "1",
                secondaryColor: "pink",
                title: "Polka",
                unit: "DOT"
            )
        )
    }
}
