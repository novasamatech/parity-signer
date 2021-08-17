//
//  TCBalance.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 16.8.2021.
//

import SwiftUI

struct TCBalance: View {
    var value: Currency
    var body: some View {
        HStack {
            Text(value.amount)
                .foregroundColor(Color("textMainColor"))
            Text(value.units).foregroundColor(Color("textMainColor"))
            Spacer()
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
    }
}

/*
struct TCBalance_Previews: PreviewProvider {
    static var previews: some View {
        TCBalance()
    }
}
*/
