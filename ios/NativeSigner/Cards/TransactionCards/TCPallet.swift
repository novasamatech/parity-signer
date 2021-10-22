//
//  TCPallet.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 23.9.2021.
//

import SwiftUI

struct TCPallet: View {
    let value: Pallet
    @State private var showDoc = false
    var body: some View {
        Button (action: {
            self.showDoc.toggle()
        }) {
            VStack {
                HStack {
                    Text("Pallet").foregroundColor(Color("AccentColor"))
                    Text(value.pallet_name)
                        .foregroundColor(Color("textMainColor"))
                    Spacer()
                    Text("?")
                        .foregroundColor(Color("AccentColor"))
                }
                if showDoc {
                    VStack {
                        Text("Path: " + value.path).foregroundColor(Color("textMainColor"))
                        Text(String(fromHexDocs: value.docs) ?? "docs parsing error in iOS, please refer to other sources")
                        .foregroundColor(Color("textMainColor"))
                        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
                    }
                }
            }
        }
    }
}

/*
struct TCPallet_Previews: PreviewProvider {
    static var previews: some View {
        TCPallet()
    }
}
*/
