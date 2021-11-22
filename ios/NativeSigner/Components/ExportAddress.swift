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
                NetworkCard(network: data.selectedNetwork)
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
            DragGesture().onEnded {drag in
                if abs(drag.translation.height) > 200 {
                    showDetails.toggle()
                } else {
                    if drag.translation.width > 20 {
                        data.selectNextAddress()
                    }
                    if drag.translation.width < -20 {
                        data.selectPreviousAddress()
                    }
                    if data.selectedAddress != nil {
                        image = data.exportIdentityQR()
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
