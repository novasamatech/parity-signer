//
//  SufficientCryptoReady.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 21.12.2021.
//

import SwiftUI

struct SufficientCryptoReady: View {
    @EnvironmentObject var data: SignerDataModel
    @GestureState private var dragOffset = CGSize.zero
    @State var offset: CGFloat = 0
    @State var oldOffset: CGFloat = 0
    var content: MSufficientCryptoReady
    var body: some View {
        ZStack{
            RoundedRectangle(cornerRadius: 8).foregroundColor(Color("Bg000"))
            VStack {
                HeaderBar(line1: "Your Signature", line2: "Scan it into your application")
                 Image(uiImage: UIImage(data: Data(fromHexEncodedString: content.sufficient) ?? Data()) ?? UIImage())
                 .resizable()
                 .aspectRatio(contentMode: .fit).padding(12)
                //AddressCard(address: content.authorInfo.intoAddress())
                Text("Payload: " + content.content.ttype)
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
