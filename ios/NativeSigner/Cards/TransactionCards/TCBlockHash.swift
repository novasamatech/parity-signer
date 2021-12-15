//
//  TCBlockHash.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 16.8.2021.
//

import SwiftUI

struct TCBlockHash: View {
    var text: String
    var body: some View {
        HStack {
            Text("Block hash: ")
                .foregroundColor(Color("Text400"))
            Text(text)
                .foregroundColor(Color("Text600"))
            Spacer()
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
    }
}

/*
struct TCBlockHash_Previews: PreviewProvider {
    static var previews: some View {
        TCBlockHash()
    }
}
*/
