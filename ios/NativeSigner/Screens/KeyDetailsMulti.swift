//
//  KeyDetailsMulti.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 6.1.2022.
//

import SwiftUI

struct KeyDetailsMulti: View {
    @GestureState private var dragOffset = CGSize.zero
    @State var offset: CGFloat = 0
    @State var showDetails = false
    var content: MKeyDetailsMulti
    let pushButton: (Action, String, String) -> Void
    var body: some View {
        ScrollView {
            VStack {
                AddressCard(address: content.keyDetails.address)
                NetworkCard(
                    title: content.keyDetails.networkInfo.networkTitle,
                    logo: content.keyDetails.networkInfo.networkLogo
                )
                Image(uiImage: UIImage(data: Data(content.keyDetails.qr)) ?? UIImage())
                    .resizable()
                    .aspectRatio(contentMode: .fit).padding(12)
                    .offset(x: offset, y: 0)
                    .onAppear {
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
                            pushButton(.nextUnit, "", "")
                        }
                        if drag.translation.width < -20 {
                            pushButton(.previousUnit, "", "")
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
