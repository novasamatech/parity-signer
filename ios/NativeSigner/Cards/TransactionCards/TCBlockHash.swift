//
//  TCBlockHash.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 16.8.2021.
//

import SwiftUI

struct TCBlockHash: View {
    var text: String
    var body: some View {
        TCNameValueTemplate(name: "Block hash", value: text)
    }
}

/*
struct TCBlockHash_Previews: PreviewProvider {
    static var previews: some View {
        TCBlockHash()
    }
}
*/
