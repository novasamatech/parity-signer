//
//  TCTypesInfo.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCTypesInfo: View {
    var content: MTypesInfo
    var body: some View {
        HStack {
            Identicon(identicon: content.typesIdPic ?? "")
            TCNameValueTemplate(name: "Types hash:", value: content.typesHash ?? "")
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
