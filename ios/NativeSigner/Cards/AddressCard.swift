//
//  IdentityCard.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 3.8.2021.
//

import SwiftUI

/**
 * Card for showing any address.
 * Accepts Address object
 */
struct AddressCard: View {
    @EnvironmentObject var data: SignerDataModel
    var address: Address
    @GestureState private var dragOffset = CGSize.zero
    let rowHeight: CGFloat = 28
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 4).foregroundColor(Color("Bg200")).frame(height: 44)
            HStack {
                Image(uiImage: UIImage(data: Data(fromHexEncodedString: address.identicon) ?? Data()) ?? UIImage())
                    .resizable(resizingMode: .stretch)
                    .frame(width: rowHeight, height: rowHeight)
                VStack (alignment: .leading) {
                    HStack {
                        Text(address.seed_name)
                            Text(address.path)
                        if address.has_pwd {
                            Text("///").foregroundColor(Color("Crypto400"))
                                .font(FCrypto(style: .body2))
                            Image(systemName: "lock").foregroundColor(Color("Crypto400"))
                                .font(FCrypto(style: .body2))
                        }
                    }.foregroundColor(Color("Crypto400"))
                        .font(FCrypto(style: .body2))
                    //Here we could have shortened base58 address when buttons are shown, but we don't need to
                    Text(address.base58.truncateMiddle(length: 8)).foregroundColor(Color("Text400"))
                        .font(FCrypto(style: .body1))
                }
                Spacer()
                /*
                if (data.keyManagerModal == .showKey && data.getMultiSelectionMode()) {
                    Text(String((data.multiSelected.firstIndex(of: address) ?? -1) + 1) + "/" + String(data.multiSelected.count))
                }*/
            }.padding(.horizontal, 8)
        }/*
        .gesture(
            TapGesture()
                .onEnded { _ in
                    if data.keyManagerModal == .showKey {
                        //we can do something here
                    } else {
                        if data.getMultiSelectionMode() {
                            data.multiSelectAction(address: address)
                        } else {
                            if address.isRoot() {
                                data.selectSeed(seedName: address.seed_name)
                            } else {
                                data.selectedAddress = address
                                data.keyManagerModal = .showKey
                            }
                        }
                    }
                })
        .gesture(
            LongPressGesture()
                .onEnded { _ in
                    if data.keyManagerModal == .showKey {
                        //we can do something here
                    } else {
                        data.multiSelectAction(address: address)
                    }
                })
        .gesture(
            DragGesture()
                .updating($dragOffset, body: { (value, state, transaction) in
                    if data.keyManagerModal == .showKey {
                        //we can do something here
                    } else {
                        if value.translation.width < -20 {
                            data.multiSelected = []
                            data.selectedAddress = address
                        }
                        if value.translation.width > 20 {
                            data.selectedAddress = nil
                        }
                    }
                })
        )*/
    }
}
