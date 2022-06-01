//
//  TCID.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCID: View {
    var value: MscId
    var body: some View {
        HStack {
            Identicon(identicon: value.identicon)
            Text(value.base58)
                .foregroundColor(Color("Text600")).font(FCrypto(style: .body2))
            Spacer()
        }
    }
}

/*
struct TCID_Previews: PreviewProvider {
    static var previews: some View {
        TCID()
    }
}
*/
