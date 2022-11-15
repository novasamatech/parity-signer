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
                .font(Fontstyle.bodyL.base)
                .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                .padding(.leading, Spacing.medium)
                .padding(.bottom, Spacing.extraExtraSmall)
            VStack {
                VStack(alignment: .leading, spacing: Spacing.small) {
                    rowWrapper(value.title)
                    rowWrapper(String(value.base58prefix))
                    rowWrapper(String(value.decimals))
                    rowWrapper(value.unit)
                    VStack(alignment: .leading, spacing: Spacing.extraSmall) {
                        Localizable.Transaction.AddNetwork.Label.genesisHash.text
                            .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                        Text(value.genesisHash.formattedAsString)
                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        Divider()
                    }
                    rowWrapper(value.encryption.rawValue)
                    rowWrapper(value.name)
                    HStack {
                        Text(value.logo)
                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                            .font(Fontstyle.header4.web3)
                    }
                    Divider()
                    rowWrapper(value.pathId, isLast: true)
                }
                .padding(Spacing.medium)
            }
            .background(Asset.fill6Solid.swiftUIColor)
            .cornerRadius(CornerRadius.medium)
        }
    }

    @ViewBuilder
    private func rowWrapper(_ value: String, isLast: Bool = false) -> some View {
        HStack {
            Text(value)
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
            Spacer()
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
                name: "Polkadot",
                pathId: "1",
                secondaryColor: "pink",
                title: "Polka",
                unit: "DOT"
            )
        )
    }
}
