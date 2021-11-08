//
//  TCPallet.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 23.9.2021.
//

import SwiftUI

struct TCPallet: View {
    let text: String
    var body: some View {
        HStack {
            Text(text)
                .foregroundColor(Color("AccentColor"))
            Spacer()
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
    }
}

/*
struct TCPallet_Previews: PreviewProvider {
    static var previews: some View {
        TCPallet()
    }
}
*/
