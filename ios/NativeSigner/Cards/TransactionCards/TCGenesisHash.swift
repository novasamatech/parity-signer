//
//  TCGenesisHash.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 7.1.2022.
//

import SwiftUI

struct TCGenesisHash: View {
    let content: String
    var body: some View {
        TCNameValueTemplate(name: "Genesis hash", value: content)
    }
}

/*
struct TCGenesisHash_Previews: PreviewProvider {
    static var previews: some View {
        TCGenesisHash()
    }
}
*/
