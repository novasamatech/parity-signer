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
    var body: some View {
        ZStack {
            ModalBackdrop()
            VStack {
                if data.selectedAddress != nil {
                    AddressCard(address: data.selectedAddress!)
                }
                if image == nil || showDetails {
                    HStack {
                        Text("Base58 key: ")
                        Text(data.selectedAddress?.ss58 ?? "unknown")
                    }.padding()
                    HStack {
                        Text("Hex key: ")
                        Text(data.selectedAddress?.public_key ?? "unknown")
                    }.padding()
                    HStack {
                        Text("Seed name: ")
                        Text(data.selectedAddress?.seed_name ?? "unknown")
                    }.padding()
                } else {
                    Image(uiImage: image!)
                        .resizable()
                        .aspectRatio(contentMode: .fit)
                }
            }
            .foregroundColor(Color("textMainColor"))
        }
        .onAppear {
            data.lastError = ""
            if data.selectedAddress != nil {
                image = data.exportIdentityQR()
            }
        }
        .onDisappear {
            data.selectedAddress = nil
        }
        .gesture(
            TapGesture()
                .onEnded {
                    print("tap")
                    data.selectNextAddress()
                    if data.selectedAddress != nil {
                        image = data.exportIdentityQR()
                    }
                }
        )
        .gesture(
            LongPressGesture()
                .onEnded { _ in
                    print("ltap")
                    data.selectPreviousAddress()
                    if data.selectedAddress != nil {
                        image = data.exportIdentityQR()
                    }
                }
        )
        .gesture(
            DragGesture().updating($dragOffset, body: {
                (value, state, transaction) in
                if value.translation.height > 300 {
                    showDetails.toggle()
                }
            })
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
