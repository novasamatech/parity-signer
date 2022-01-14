//
//  TCAuthorPublicKey.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 16.8.2021.
//

import SwiftUI

struct TCAuthorPublicKey: View {
    var value: AuthorPublicKey
    var body: some View {
        HStack {
            Image(systemName: "circle.fill").foregroundColor(Color("Text600")).imageScale(.large)
            VStack (alignment: .leading) {
                Text("Signed with " + value.crypto)
                    .foregroundColor(Color("Text400")).font(FBase(style: .body2))
                Text(value.hex)
                    .font(FCrypto(style: .body2))
                    .foregroundColor(Color("Crypto400"))
            }
            Spacer()
        }
    }
}

/*
struct TCAuthorPublicKey_Previews: PreviewProvider {
    static var previews: some View {
        TCAuthorPublicKey()
    }
}
*/
