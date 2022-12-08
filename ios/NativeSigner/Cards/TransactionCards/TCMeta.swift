//
//  TCMeta.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 7.1.2022.
//

import SwiftUI

struct TCMeta: View {
    let value: MMetadataRecord
    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            Localizable.Transaction.Metadata.Label.header.text
                .font(Fontstyle.bodyL.base)
                .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                .padding(.leading, Spacing.medium)
                .padding(.bottom, Spacing.extraExtraSmall)
            VStack {
                VStack(spacing: Spacing.small) {
                    VStack(alignment: .leading) {
                        HStack {
                            Localizable.Transaction.Metadata.Label.metadata.text
                                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                            Spacer()
                            Text(value.specname)
                                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                            Text(value.specsVersion)
                                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        }
                    }
                    Divider()
                    Text(value.metaHash)
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                }
                .padding(Spacing.medium)
            }
            .background(Asset.fill6Solid.swiftUIColor)
            .cornerRadius(CornerRadius.medium)
        }
    }
}

struct TCMeta_Previews: PreviewProvider {
    static var previews: some View {
        TCMeta(value: MMetadataRecord(
            specname: "Westend",
            specsVersion: "9230",
            metaHash: "5DCmwXp8XLzSMUyE4uhJMKV4vwvsWqqBYFKJq38CW53VHEVq",
            metaIdPic: .svg(image: PreviewData.exampleIdenticon)
        ))
    }
}
