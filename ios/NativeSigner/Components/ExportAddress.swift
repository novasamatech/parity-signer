//
//  ExportIdentity.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 4.8.2021.
//

import SwiftUI

struct ExportAddress: View {
    @EnvironmentObject var data: SignerDataModel
    @State var image: UIImage?
    var body: some View {
        ZStack {
            ModalBackdrop()
            VStack {
                if data.selectedAddress != nil {
                    AddressCard(address: data.selectedAddress!)
                }
                if image != nil {
                    Image(uiImage: image!)
                        .resizable()
                        .aspectRatio(contentMode: .fit)
                }
            }
            .foregroundColor(/*@START_MENU_TOKEN@*/Color("textMainColor")/*@END_MENU_TOKEN@*/)
        }
        .onAppear {
            data.lastError = ""
            if data.selectedAddress != nil {
                image = data.exportIdentityQR()
            }
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
    }
}

/*
 struct ExportIdentity_Previews: PreviewProvider {
 static var previews: some View {
 ExportAddress()
 }
 }
 */
