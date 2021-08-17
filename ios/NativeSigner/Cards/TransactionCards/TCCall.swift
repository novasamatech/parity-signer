//
//  TCCall.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCCall: View {
    let value: Call
    var body: some View {
        HStack {
            Text(value.method)
                .foregroundColor(Color("textMainColor"))
            Text(" from ")
                .foregroundColor(Color("AccentColor"))
            Text(value.pallet)
                .foregroundColor(Color("textMainColor"))
            Spacer()
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
    }
}

/*
struct TCCall_Previews: PreviewProvider {
    static var previews: some View {
        TCCall()
    }
}
*/
