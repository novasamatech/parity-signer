//
//  TCAuthorPublicKey.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 16.8.2021.
//

import SwiftUI

struct TCAuthorPublicKey: View {
    var value: MVerifierDetails
    var body: some View {
        HStack {
            Identicon(identicon: value.identicon)
            VStack(alignment: .leading) {
                Text("Signed with " + value.encryption)
                    .foregroundColor(Color("Text400")).font(Fontstyle.body2.base)
                Text(value.publicKey)
                    .font(Fontstyle.body2.crypto)
                    .foregroundColor(Color("Crypto400"))
            }
            Spacer()
        }
    }
}

// struct TCAuthorPublicKey_Previews: PreviewProvider {
//    static var previews: some View {
//        TCAuthorPublicKey()
//    }
// }
