//
//  TCTXSpecPlain.swift
//  Polkadot Vault
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
                    .foregroundColor(.textAndIconsTertiary)
                    .font(PrimaryFont.bodyL.font)
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

#if DEBUG
    struct TCTXSpecPlain_Previews: PreviewProvider {
        static var previews: some View {
            TCTXSpecPlain(content: .stub)
        }
    }
#endif
