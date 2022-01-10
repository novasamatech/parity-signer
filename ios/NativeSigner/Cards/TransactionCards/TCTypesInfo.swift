//
//  TCTypesInfo.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCTypesInfo: View {
    var content: TypesInfo
    var body: some View {
        HStack {
            Identicon(identicon: content.types_id_pic)
            TCNameValueTemplate(name: "Types hash:", value: content.types_hash)
        }
    }
}

/*
 struct TCTypesInfo_Previews: PreviewProvider {
 static var previews: some View {
 TCTypesInfo()
 }
 }
 */
