//
//  TCTXSpecPlain.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 7.1.2022.
//

import SwiftUI

struct TCTXSpecPlain: View {
    let content: MscTxSpecPlain
    var body: some View {
        HStack {
            VStack(alignment: .leading, spacing: Spacing.extraSmall) {
                Localizable.unknownNetwork.text
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    .font(Fontstyle.bodyL.base)
                TCNamedValueCard(
                    name: Localizable.TCName.genesisHash.string,
                    value: content.networkGenesisHash.formattedAsString
                )
                TCNamedValueCard(name: Localizable.TCName.version.string, value: content.version)
                TCNamedValueCard(name: Localizable.TCName.txVersion.string, value: content.txVersion)
            }
            Spacer()
        }
    }
}

struct TCTXSpecPlain_Previews: PreviewProvider {
    static var previews: some View {
        TCTXSpecPlain(content: MscTxSpecPlain(
            networkGenesisHash: .init([3, 4, 5]),
            version: "9230",
            txVersion: "tx9230"
        ))
    }
}
