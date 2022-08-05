//
//  ExportIdentity.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 4.8.2021.
//

import SwiftUI

struct ExportAddress: View {
    @State var showDetails = false
    var content: MKeyDetails
    var body: some View {
        ScrollView {
            VStack {
                AddressCard(address: content.address)
                Image(uiImage: UIImage(data: Data(content.qr)) ?? UIImage())
                    .resizable()
                    .aspectRatio(contentMode: .fit).padding(12)
                HeaderBar(line1: "KEY DETAILS", line2: "").padding(.horizontal, 8)
                VStack {
                    HStack {
                        Text("Base58 key: ")
                        Text(content.address.base58)
                    }.padding().foregroundColor(Color("Crypto400")).font(FCrypto(style: .body2))
                    HStack {
                        Text("Hex key: ")
                        Text(content.pubkey)
                    }.padding().foregroundColor(Color("Crypto400")).font(FCrypto(style: .body2))
                    HStack {
                        Text("Seed name: ")
                        Text(content.address.seedName)
                    }.padding().foregroundColor(Color("Text400")).font(FBase(style: .body2))
                }
            }
        }
    }
}

/*
 struct ExportIdentity_Previews: PreviewProvider {
 static var previews: some View {
 ExportAddress()
 }
 }
 */
