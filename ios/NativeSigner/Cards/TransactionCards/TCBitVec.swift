//
//  TCBitVec.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 7.1.2022.
//

import SwiftUI

struct TCBitVec: View {
    let content: String
    var body: some View {
        TCNameValueTemplate(name: "BitVec", value: content)
    }
}

/*
struct TCBitVec_Previews: PreviewProvider {
    static var previews: some View {
        TCBitVec()
    }
}
*/
