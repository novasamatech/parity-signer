//
//  KeyDetailsMulti.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 6.1.2022.
//

import SwiftUI

struct KeyDetailsMulti: View {
    @GestureState private var dragOffset = CGSize.zero
    @State private var offset: CGFloat = 0
    @State private var showDetails = false
    var content: MKeyDetailsMulti
    let navigationRequest: NavigationRequest
    var body: some View {
        ScrollView {
            VStack {
                AddressCard(card: MAddressCard(
                    base58: content.keyDetails.base58,
                    addressKey: "",
                    address: content.keyDetails.address,
                    multiselect: nil
                ))
                NetworkCard(
                    title: content.keyDetails.networkInfo.networkTitle,
                    logo: content.keyDetails.networkInfo.networkLogo
                )
                Image(uiImage: UIImage(data: Data(content.keyDetails.qr.payload)) ?? UIImage())
                    .resizable()
                    .aspectRatio(contentMode: .fit).padding(12)
                    .offset(x: offset, y: 0)
                    .onAppear {
                        offset = 0
                    }
                Text(Localizable.Key.outOf(content.currentNumber, content.outOf))
            }
        }
        .gesture(
            DragGesture()
                .onChanged { drag in
                    self.offset = drag.translation.width
                }
                .onEnded { drag in
                    self.offset = 0
                    if abs(drag.translation.height) > 200 {
                        showDetails.toggle()
                    } else {
                        if drag.translation.width > 20 {
                            navigationRequest(.init(action: .nextUnit))
                        }
                        if drag.translation.width < -20 {
                            navigationRequest(.init(action: .previousUnit))
                        }
                    }
                }
        )
    }
}

// struct KeyDetailsMulti_Previews: PreviewProvider {
// static var previews: some View {
// KeyDetailsMulti()
// }
// }
