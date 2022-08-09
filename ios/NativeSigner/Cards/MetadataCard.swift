//
//  MetadataCard.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 12.8.2021.
//

import SwiftUI

struct MetadataCard: View {
    var meta: MMetadataRecord
    var body: some View {
        HStack {
            Identicon(identicon: meta.metaIdPic) // this is potentially different from identicon
            VStack {
                Localizable.version.text
                Text(meta.specsVersion)
            }
            VStack {
                Localizable.hash.text
                Text(meta.metaHash.truncateMiddle(length: 8))
            }
        }.padding(.horizontal, 8)
    }
}

// struct MetadataCard_Previews: PreviewProvider {
//    static var previews: some View {
//        MetadataCard()
//    }
// }
