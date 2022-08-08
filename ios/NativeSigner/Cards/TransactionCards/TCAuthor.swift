//
//  TCAuthor.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 16.8.2021.
//

import SwiftUI

struct TCAuthor: View {
    var author: Address
    var body: some View {
        HStack {
            Identicon(identicon: author.identicon)
            VStack(alignment: .leading) {
                Text("From:")
                    .foregroundColor(Asset.text400.swiftUIColor)
                Text(author.seedName + author.path + (author.hasPwd == true ? "///" : ""))
                    .foregroundColor(Asset.crypto400.swiftUIColor)
                Text(author.base58)
                    .font(.caption2)
                    .foregroundColor(Asset.text600.swiftUIColor)
            }
            Spacer()
        }
    }
}

// struct TCAuthor_Previews: PreviewProvider {
//    static var previews: some View {
//        TCAuthor()
//    }
// }
