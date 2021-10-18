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
    let rowHeight: CGFloat = 42
    var body: some View {
        VStack {
            HStack {
                Image(uiImage: UIImage(data: Data(fromHexEncodedString: String(cString: base58_identicon(nil, address.ss58, 32))) ?? Data()) ?? UIImage())
                    .resizable(resizingMode: .stretch)
                    .frame(width: rowHeight, height: rowHeight)
                VStack (alignment: .leading) {
                    HStack {
                        if address.isRoot() {
                            Text(address.seed_name)
                                .foregroundColor(Color("textMainColor"))
                                .font(.headline)
                        } else {
                            Text(address.path)
                                .foregroundColor(Color("cryptoColor"))
                                .font(.headline)
                        }
                        if address.has_password == "true" {
                            Image(systemName: "lock")
                                .foregroundColor(Color("AccentColor"))
                        }
                    }
                    Text((data.selectedAddress == address && data.keyManagerModal == .none) ? address.truncateBase58to8() : address.truncateBase58())
                        .font(.headline)
                        .foregroundColor(Color("textFadedColor"))
                }
                Spacer()
                if data.keyManagerModal == .none {
                    AddressCardControls(address: address, rowHeight: rowHeight)
                }
                if (data.keyManagerModal == .showKey && data.getMultiSelectionMode()) {
                    Text(String((data.multiSelected.firstIndex(of: address) ?? -1) + 1) + "/" + String(data.multiSelected.count))
                }
            }
            
        }
        .padding(8)
        .gesture(
            TapGesture()
                .onEnded { _ in
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
                })
        .gesture(
            LongPressGesture()
                .onEnded { _ in
                    data.multiSelectAction(address: address)})
        .gesture(
            DragGesture()
                .updating($dragOffset, body: { (value, state, transaction) in
                    if value.translation.width < -20 {
                        data.multiSelected = []
                        data.selectedAddress = address
                    }
                    if value.translation.width > 20 {
                        data.selectedAddress = nil
                    }
                })
        )
        .background(Color(data.selectedAddress == address ? "backgroundActive" : "backgroundCard"))
    }
}

/*
 struct IdentityCard_Previews: PreviewProvider {
 static var previews: some View {
 IdentityCard(identity: Identity.identityData[0]).previewLayout(.sizeThatFits)
 }
 }
 */
