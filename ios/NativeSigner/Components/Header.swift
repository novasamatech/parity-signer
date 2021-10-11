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
            if ((data.transactionState != .none) && (data.signerScreen != .scan)) {
                Button(action: {
                    data.totalRefresh()
                    data.signerScreen = .scan
                }) {
                    Text("Back")
                }}
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
            Button(action: {
                data.totalRefresh()
                data.networkSettings = nil
                data.signerScreen = .settings
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
