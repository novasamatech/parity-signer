//
//  TCAuthor.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 16.8.2021.
//

import SwiftUI

struct TCAuthor: View {
    var author: TransactionAuthor
    var body: some View {
        HStack {
            Identicon(identicon: author.identicon)
            VStack (alignment: .leading) {
                Text("From:")
                    .foregroundColor(Color("Text400"))
                Text(author.seed.decode64() + author.derivationPath + (author.has_password == true ? "///" : ""))
                    .foregroundColor(Color("Crypto400"))
                Text(author.base58)
                    .font(.caption2)
                    .foregroundColor(Color("Text600"))
            }
            Spacer()
        }
    }
}
 /*
struct TCAuthor_Previews: PreviewProvider {
    static var previews: some View {
        TCAuthor()
    }
}*/
