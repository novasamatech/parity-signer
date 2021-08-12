//
//  MetadataCard.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 12.8.2021.
//

import SwiftUI

struct MetadataCard: View {
    var meta: MetaSpecsNS
    var body: some View {
        VStack(alignment: .leading) {
            HStack {
                Text("spec_version:")
                    .foregroundColor(Color("AccentColor"))
                Text(meta.spec_version)
                    .foregroundColor(Color("textMainColor"))
            }
            HStack {
                Text("metadata hash:")
                    .foregroundColor(Color("AccentColor"))
                Text(meta.meta_hash)
                    .foregroundColor(Color("textMainColor"))
            }
        }.padding()
    }
}

/*
struct MetadataCard_Previews: PreviewProvider {
    static var previews: some View {
        MetadataCard()
    }
}
*/
