//
//  TCMeta.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 7.1.2022.
//

import SwiftUI

struct TCMeta: View {
    let content: MetaSpecs
    var body: some View {
        HStack {
            Identicon(identicon: content.meta_id_pic)
            VStack{
                Text("Add metadata").foregroundColor(Color("Text600"))
                HStack {
                    Text(content.specname)
                    Text(content.spec_version)
                }
                .foregroundColor(Color("Crypto400")).font(FCrypto(style: .body2))
                Text(content.meta_hash).foregroundColor(Color("Text400")).font(FCrypto(style: .body2))
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
