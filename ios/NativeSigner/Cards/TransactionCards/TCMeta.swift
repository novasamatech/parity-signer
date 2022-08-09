//
//  TCMeta.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 7.1.2022.
//

import SwiftUI

struct TCMeta: View {
    let content: MMetadataRecord
    var body: some View {
        HStack {
            Identicon(identicon: content.metaIdPic)
            VStack {
                Localizable.addMetadata.text
                    .foregroundColor(Asset.text600.swiftUIColor)
                HStack {
                    Text(content.specname)
                    Text(content.specsVersion)
                }
                .foregroundColor(Asset.crypto400.swiftUIColor).font(Fontstyle.body2.crypto)
                Text(content.metaHash).foregroundColor(Asset.text400.swiftUIColor).font(Fontstyle.body2.crypto)
            }
            Spacer()
        }
    }
}

// struct TCMeta_Previews: PreviewProvider {
// static var previews: some View {
// TCMeta()
// }
// }
