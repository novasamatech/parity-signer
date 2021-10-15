//
//  Header.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 13.9.2021.
//

import SwiftUI

struct Header: View {
    @EnvironmentObject var data: SignerDataModel
    var body: some View {
        HStack {
            if !data.isNavBottom() {
                Button(action: {
                    data.goBack()
                }) {
                    Image(systemName: "chevron.left").imageScale(.large)
                }
            }
            if data.getMultiSelectionMode() && data.keyManagerModal == .none {
                Button(action: {data.multiSelected = []}) {
                    SmallButton(text: "Cancel")
                }
            }
            Spacer()
            Text(data.getScreenName())
            Spacer()
            if data.getMultiSelectionMode() && data.keyManagerModal == .none {
                Button(action: {data.multiSelected = data.addresses}) {
                    SmallButton(text: "Select all")
                }
            }
            if (data.keyManagerModal == .seedSelector && data.signerScreen == .keys) {
                Button(action: {
                    data.keyManagerModal = .newSeed
                }) {
                    Image(systemName: "plus.square.on.square")
                        .imageScale(.large)
                }
            }
            Button(action: {
                data.totalRefresh()
                data.networkSettings = nil
                data.signerScreen = .history
            }) {
                NavbarShield()
            }
        }
        .padding().background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
    }
}

/*
 struct Header_Previews: PreviewProvider {
 static var previews: some View {
 Header().previewLayout(.sizeThatFits)
 }
 }
 */
