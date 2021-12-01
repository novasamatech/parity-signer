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
            RoundedRectangle(cornerRadius: 4).foregroundColor(Color(false ? "backgroundActive" : "backgroundCard")).frame(height: 44)
            HStack {
                Image(uiImage: UIImage(data: Data(fromHexEncodedString: String(cString: base58_identicon(nil, address.ss58, 32))) ?? Data()) ?? UIImage())
                    .resizable(resizingMode: .stretch)
                    .frame(width: rowHeight, height: rowHeight)
                VStack (alignment: .leading) {
                    HStack {
                        if address.isRoot() {
                            Text(address.seed_name)
                                .foregroundColor(Color("textMainColor"))
                        } else {
                            Text(address.path)
                                .foregroundColor(Color("cryptoColor"))
                        }
                        if address.has_password == "true" {
                            Image(systemName: "lock")
                                .foregroundColor(Color("AccentColor"))
                        }
                    }.font(.system(size: 12, weight: .semibold, design: .monospaced))
                    //Here we could have shortened base58 address when buttons are shown, but we don't need to
                    Text(address.truncateBase58())
                        .foregroundColor(Color("textFadedColor"))
                        .font(.system(size: 12, design: .monospaced))
                }
                Spacer()
                if data.keyManagerModal == .none {
                    AddressCardControls(address: address, rowHeight: rowHeight+11)
                }
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
