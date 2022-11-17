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
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    .font(Fontstyle.bodyL.base)
                Text(author.formattedAddress)
                    .foregroundColor(Asset.accentPink300.swiftUIColor)
                    .font(Fontstyle.bodyL.base)
                Text(author.base58)
                    .font(Fontstyle.captionS.base)
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
            }
            Spacer()
        }
    }
}

private extension MAddressCard {
    var formattedAddress: String {
        [address.seedName, address.path, address.hasPwd == true ? Localizable.Path.delimeter.string : ""].joined()
    }
}

struct TCAuthor_Previews: PreviewProvider {
    static var previews: some View {
        TCAuthor(
            author: MAddressCard(
                base58: "5F3sa2TJAWMqDhXG6jhV4N8ko9SxwGy8TpaNS1repo5EYjQX",
                address: PreviewData.address,
                multiselect: true
            )
        )
    }
}
