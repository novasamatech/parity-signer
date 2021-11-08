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
        HStack {
            VStack {
                Text("version")
                    .foregroundColor(Color("AccentColor"))
                Text(meta.spec_version)
                    .foregroundColor(Color("textMainColor"))
                Spacer()
            }
            VStack {
                Text("hash")
                    .foregroundColor(Color("AccentColor"))
                Text(meta.meta_hash)
                    .foregroundColor(Color("textMainColor"))
                    .font(.caption2)
            }
        }.padding(.horizontal)
    }
}

/*
struct MetadataCard_Previews: PreviewProvider {
    static var previews: some View {
        MetadataCard()
    }
}
*/
