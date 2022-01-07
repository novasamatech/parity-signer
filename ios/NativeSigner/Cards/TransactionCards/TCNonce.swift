//
//  TCNonce.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 7.1.2022.
//

import SwiftUI

struct TCNonce: View {
    let content: String
    var body: some View {
        TCNameValueTemplate(name: "Nonce ", value: content)
    }
}

/*
struct TCNonce_Previews: PreviewProvider {
    static var previews: some View {
        TCNonce()
    }
}
*/
