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
            Image(systemName: "circle.fill").foregroundColor(Color("Action400")).imageScale(.large)
            VStack (alignment: .leading) {
                Text("Signed with " + value.crypto)
                    .foregroundColor(Color("Action400"))
                Text(value.hex)
                    .font(.caption2)
                    .foregroundColor(Color("Text600"))
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
