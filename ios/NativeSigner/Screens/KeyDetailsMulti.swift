//
//  KeyDetailsMulti.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 6.1.2022.
//

import SwiftUI

struct KeyDetailsMulti: View {
    @EnvironmentObject var data: SignerDataModel
    @GestureState private var dragOffset = CGSize.zero
    @State var offset: CGFloat = 0
    @State var showDetails = false
    var content: MKeyDetailsMulti
    var body: some View {
        ScrollView {
            VStack {
                AddressCard(address: Address(base58: content.keyDetails.base58, path: content.keyDetails.path, hasPwd: false /*TODO*/, identicon: content.keyDetails.identicon, seedName: content.keyDetails.seedName, multiselect: false))
                NetworkCard(title: content.keyDetails.networkTitle, logo: content.keyDetails.networkLogo)
                Image(uiImage: UIImage(data: Data(fromHexEncodedString: content.keyDetails.qr) ?? Data()) ?? UIImage())
                    .resizable()
                    .aspectRatio(contentMode: .fit).padding(12)
                    .offset(x: offset, y:0)
                    .onAppear{
                        offset = 0
                    }
                Text("Key " + content.currentNumber + " out of " + content.outOf)
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
                            data.pushButton(action: .nextUnit)
                        }
                        if drag.translation.width < -20 {
                            data.pushButton(action: .previousUnit)
                        }
                    }
                }
        )
    }
}

/*
 struct KeyDetailsMulti_Previews: PreviewProvider {
 static var previews: some View {
 KeyDetailsMulti()
 }
 }
 */
