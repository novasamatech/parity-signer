//
//  TCAuthor.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 16.8.2021.
//

import SwiftUI

struct TCAuthor: View {
    var author: MAddressCard
    var body: some View {
        HStack {
            Identicon(identicon: author.address.identicon)
            VStack(alignment: .leading) {
                Localizable.from.text
                    .foregroundColor(Asset.text400.swiftUIColor)
                Text(
                    author.address.seedName + author.address
                        .path + (author.address.hasPwd == true ? Localizable.Path.delimeter.string : "")
                )
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
