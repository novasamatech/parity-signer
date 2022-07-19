//
//  SufficientCryptoReady.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 21.12.2021.
//

import SwiftUI

struct SufficientCryptoReady: View {
    @GestureState private var dragOffset = CGSize.zero
    @State var offset: CGFloat = 0
    @State var oldOffset: CGFloat = 0
    var content: MSufficientCryptoReady
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 8).foregroundColor(Color("Bg000"))
            VStack {
                HeaderBar(line1: "Your Signature", line2: "Scan it into your application")
                 Image(uiImage: UIImage(data: Data(content.sufficient)) ?? UIImage())
                 .resizable()
                 .aspectRatio(contentMode: .fit).padding(12)
                AddressCard(address: content.authorInfo)
                switch content.content {
                case .addSpecs(let network):
                    Text("Signature for network specs")
                    NetworkCard(title: network.networkTitle, logo: network.networkLogo)
                case .loadTypes(types: _, pic: _):
                    Text("Signature for types")
                case .loadMetadata(name: let name, version: let version):
                    Text("Signature for metadata update" )
                    Text(name + " version " + String(version))
                }
            }
        }
    }
}

/*
 struct SufficientCryptoReady_Previews: PreviewProvider {
 static var previews: some View {
 SufficientCryptoReady()
 }
 }
 */
