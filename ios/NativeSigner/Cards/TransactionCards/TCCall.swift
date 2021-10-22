//
//  TCCall.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCCall: View {
    let value: Call
    @State private var showDoc = false
    var body: some View {
        Button (action: {
            self.showDoc.toggle()
        }) {
            VStack {
                HStack {
                    Text(value.method)
                        .foregroundColor(Color("textMainColor"))
                    Text(" from ")
                        .foregroundColor(Color("AccentColor"))
                    Text(value.pallet)
                        .foregroundColor(Color("textMainColor"))
                    Spacer()
                    if value.docs != "" {
                        Text("?")
                        .foregroundColor(Color("AccentColor"))
                    }
                }
                if showDoc {
                    Text(String(fromHexDocs: value.docs) ?? "docs parsing error in iOS, please refer to other sources")
                        .foregroundColor(Color("textMainColor"))
                        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
                }
            }
        }.disabled(value.docs == "")
    }
}

/*
 struct TCCall_Previews: PreviewProvider {
 static var previews: some View {
 TCCall()
 }
 }
 */
