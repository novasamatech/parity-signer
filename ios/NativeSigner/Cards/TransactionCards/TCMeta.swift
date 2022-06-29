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
                Text("Add metadata").foregroundColor(Color("Text600"))
                HStack {
                    Text(content.specname)
                    Text(content.specsVersion)
                }
                .foregroundColor(Color("Crypto400")).font(FCrypto(style: .body2))
                Text(content.metaHash).foregroundColor(Color("Text400")).font(FCrypto(style: .body2))
            }
            Spacer()
        }
    }
}

/*
 struct TCMeta_Previews: PreviewProvider {
 static var previews: some View {
 TCMeta()
 }
 }
 */
