//
//  ExportIdentity.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 4.8.2021.
//

import SwiftUI

struct ExportAddress: View {
    @EnvironmentObject var data: SignerDataModel
    @State var showDetails = false
    var content: MKeyDetails
    var body: some View {
        ScrollView {
            VStack {
                AddressCard(address: Address(base58: content.base58, path: content.path, hasPwd: false /*TODO*/, identicon: content.identicon, seedName: content.seedName, multiselect: nil))
                NetworkCard(title: content.networkTitle, logo: content.networkLogo)
                Image(uiImage: UIImage(data: Data(fromHexEncodedString: content.qr) ?? Data()) ?? UIImage())
                    .resizable()
                    .aspectRatio(contentMode: .fit).padding(12)
                HeaderBar(line1: "KEY DETAILS", line2: "").padding(.horizontal, 8)
                VStack {
                    HStack {
                        Text("Base58 key: ")
                        Text(content.base58)
                    }.padding().foregroundColor(Color("Crypto400")).font(FCrypto(style: .body2))
                    HStack {
                        Text("Hex key: ")
                        Text(content.pubkey)
                    }.padding().foregroundColor(Color("Crypto400")).font(FCrypto(style: .body2))
                    HStack {
                        Text("Seed name: ")
                        Text(content.seedName.decode64())
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
