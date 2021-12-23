//
//  ExportIdentity.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 4.8.2021.
//

import SwiftUI

struct ExportAddress: View {
    @EnvironmentObject var data: SignerDataModel
    @GestureState private var dragOffset = CGSize.zero
    @State var offset: CGFloat = 0
    @State var image: UIImage?
    @State var showDetails = false
    var content: MKeyDetails
    var body: some View {
        ScrollView {
            VStack {
                AddressCard(address: content.intoAddress())
                NetworkCard(title: content.network_title, logo: content.network_logo)
                Image(uiImage: UIImage(data: Data(fromHexEncodedString: content.qr) ?? Data()) ?? UIImage())
                    .resizable()
                    .aspectRatio(contentMode: .fit).padding(12)
                    .offset(x: offset, y:0)
                    .onAppear{
                        offset = 0
                    }
                HeaderBar(line1: "KEY DETAILS", line2: "Key key details details").padding(.horizontal, 8)
                VStack {
                    HStack {
                        Text("Base58 key: ")
                        Text(content.base58)
                    }.padding()
                    HStack {
                        Text("Hex key: ")
                        Text(content.pubkey)
                    }.padding()
                    HStack {
                        Text("Seed name: ")
                        Text(content.seed_name)
                    }.padding()
                }
                .foregroundColor(Color("Crypto400"))
            }
        }
        .gesture(
            DragGesture()
                .onChanged {drag in
                    self.offset = drag.translation.width
                }
                .onEnded {drag in
                    self.offset = 0
                    if abs(drag.translation.height) > 200 {
                        showDetails.toggle()
                    } else {
                        if drag.translation.width > 20 {
                            data.pushButton(buttonID: .NextUnit)
                        }
                        if drag.translation.width < -20 {
                            data.pushButton(buttonID: .PreviousUnit)
                        }
                    }
                }
        )
    }
}

/*
 struct ExportIdentity_Previews: PreviewProvider {
 static var previews: some View {
 ExportAddress()
 }
 }
 */
