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
                .foregroundColor(Color("Text600"))
            Text(value.units).foregroundColor(Color("Text600"))
            Spacer()
        }
    }
}

/*
struct TCBalance_Previews: PreviewProvider {
    static var previews: some View {
        TCBalance()
    }
}
*/
