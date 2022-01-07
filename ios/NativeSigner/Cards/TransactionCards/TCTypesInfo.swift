//
//  TCTypesInfo.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCTypesInfo: View {
    var text: String
    var body: some View {
        TCNameValueTemplate(name: "Types hash:", value: text)
    }
}

/*
struct TCTypesInfo_Previews: PreviewProvider {
    static var previews: some View {
        TCTypesInfo()
    }
}
*/
