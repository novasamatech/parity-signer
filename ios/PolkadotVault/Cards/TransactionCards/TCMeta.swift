//
//  TCMeta.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 7.1.2022.
//

import SwiftUI

struct TCMeta: View {
    let value: MMetadataRecord
    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            Localizable.Transaction.Metadata.Label.header.text
                .font(PrimaryFont.bodyL.font)
                .foregroundColor(.textAndIconsSecondary)
                .padding(.leading, Spacing.medium)
                .padding(.bottom, Spacing.extraExtraSmall)
            VStack(spacing: Spacing.small) {
                VStack(alignment: .leading) {
                    HStack {
                        Localizable.Transaction.Metadata.Label.metadata.text
                            .foregroundColor(.textAndIconsTertiary)
                        Spacer()
                        Text(value.specname)
                            .foregroundColor(.textAndIconsPrimary)
                        Text(value.specsVersion)
                            .foregroundColor(.textAndIconsPrimary)
                    }
                }
                Divider()
                Text(value.metaHash)
                    .foregroundColor(.textAndIconsPrimary)
            }
            .verticalRoundedBackgroundContainer()
        }
    }
}

#if DEBUG
    struct TCMeta_Previews: PreviewProvider {
        static var previews: some View {
            TCMeta(value: .stub)
        }
    }
#endif
