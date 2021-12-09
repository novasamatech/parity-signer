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
                HeaderBar(line1: "KEY DETAILS", line2: "Key key details details")
                VStack {
                    HStack {
                        Text("Base58 key: ")
                        //Text(data.selectedAddress?.ss58 ?? "unknown")
                    }.padding()
                    HStack {
                        Text("Hex key: ")
                        //Text(data.selectedAddress?.public_key ?? "unknown")
                    }.padding()
                    HStack {
                        Text("Seed name: ")
                        //Text(data.selectedAddress?.seed_name ?? "unknown")
                    }.padding()
                }
                .foregroundColor(Color("Crypto400"))
            }
        }
        .gesture(
            DragGesture().onEnded {drag in
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
