//
//  ExportIdentity.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 4.8.2021.
//

import SwiftUI

struct ExportAddress: View {
    @State private var showDetails = false
    var content: MKeyDetails
    var body: some View {
        ScrollView {
            VStack {
                AddressCard(address: content.address)
                Image(uiImage: UIImage(data: Data(content.qr)) ?? UIImage())
                    .resizable()
                    .aspectRatio(contentMode: .fit).padding(12)
                HeaderBar(line1: Localizable.ExportAddress.header.key, line2: "").padding(.horizontal, 8)
                VStack {
                    HStack {
                        Localizable.base58Key.text
                        Text(content.address.base58)
                    }.padding().foregroundColor(Asset.crypto400.swiftUIColor).font(Fontstyle.body2.crypto)
                    HStack {
                        Localizable.hexKey.text
                        Text(content.pubkey)
                    }.padding().foregroundColor(Asset.crypto400.swiftUIColor).font(Fontstyle.body2.crypto)
                    HStack {
                        Localizable.seedNameAlt.text
                        Text(content.address.seedName)
                    }.padding().foregroundColor(Asset.text400.swiftUIColor).font(Fontstyle.body2.base)
                }
            }
        }
    }
}

// struct ExportIdentity_Previews: PreviewProvider {
// static var previews: some View {
// ExportAddress()
// }
// }
