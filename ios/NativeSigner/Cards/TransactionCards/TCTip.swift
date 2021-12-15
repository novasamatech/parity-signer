//
//  TCTip.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCTip: View {
    var value: Currency
    var body: some View {
        HStack {
            Text("Tip: ")
                .foregroundColor(Color("Text400"))
            Text(value.amount)
                .foregroundColor(Color("Text600"))
            Text(value.units).foregroundColor(Color("Text600"))
            Spacer()
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
    }
}

/*
struct TCTip_Previews: PreviewProvider {
    static var previews: some View {
        TCTip()
    }
}
 */
