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
                    Text("Back")
                }}
            if data.getMultiSelectionMode() && data.keyManagerModal == .none {
                Button(action: {data.multiSelected = []}) {
                    Text("Cancel")
                }
            }
            Spacer()
            switch data.signerScreen {
            case .scan:
                switch data.transactionState {
                case .none:
                    Text("Home")
                case .parsing:
                    Text("Parsing")
                case .preview:
                    Text("Payload")
                case .password:
                    Text("Password")
                case .signed:
                    Text("Scan to publish")
                }
            case .keys:
                Text("Key manager")
            case .settings:
                Text("Settings")
            case .history:
                Text("History")
            }
            Spacer()
            if data.getMultiSelectionMode() && data.keyManagerModal == .none {
                Button(action: {data.multiSelected = data.addresses}) {
                    Text("Select all")
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

struct Header_Previews: PreviewProvider {
    static var previews: some View {
        Header().previewLayout(.sizeThatFits)
    }
}
