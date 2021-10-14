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
    var body: some View {
        VStack {
            HStack {
                Image(uiImage: UIImage(data: Data(fromHexEncodedString: String(cString: base58_identicon(nil, address.ss58, 32))) ?? Data()) ?? UIImage())
                    .resizable(resizingMode: .stretch)
                    .frame(width: 42, height: 42)
                VStack (alignment: .leading) {
                    HStack {
                        Text(address.seed_name)
                            .foregroundColor(Color("textMainColor"))
                            .font(.headline)
                        Text(address.path)
                            .foregroundColor(Color("cryptoColor"))
                            .font(.headline)
                        if address.has_password == "true" {
                            Image(systemName: "lock")
                                .foregroundColor(Color("AccentColor"))
                        }
                    }
                    Text(address.truncateBase58())
                        .font(.headline)
                        .foregroundColor(Color("textFadedColor"))
                }
                Spacer()
                if data.getMultiSelectionMode() {
                    if data.multiSelected.contains(address) {
                        Image(systemName: "checkmark.circle.fill")
                    } else {
                        Image(systemName: "circle")
                    }
                }
            }
            
        }
        .padding(8)
        .gesture(TapGesture()
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
        .gesture(LongPressGesture()
                    .onEnded { _ in
            data.multiSelectAction(address: address)})
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

/*
 if data.selectedAddress == address {
 HStack{
 Button(action: {
 //
 delete = true
 }) {
 Text("Delete")
 }
 .alert(isPresented: $delete, content: {
 Alert(
 title: Text("Delete key?"),
 message: Text("You are about to delete key " + data.selectedAddress!.name),
 primaryButton: .cancel(),
 secondaryButton: .destructive(
 Text("Delete"),
 action: { data.deleteSelectedAddress()
 }
 )
 )
 })
 Spacer()
 Button(action: {
 data.keyManagerModal = .showKey
 }) {
 Text("Export")
 }
 Spacer()
 Button(action: {
 data.selectSeed(seedName: data.selectedAddress!.seed_name)
 data.proposeIncrement()
 data.keyManagerModal = .newKey
 }) {
 Text("N+1")
 }
 Spacer()
 Button(action: {
 data.selectSeed(seedName: data.selectedAddress!.seed_name)
 data.proposeDerive()
 data.keyManagerModal = .newKey
 }) {
 Text("Derive")
 }
 }
 }*/
