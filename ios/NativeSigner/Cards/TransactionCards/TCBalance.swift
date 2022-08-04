//
//  TCBalance.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 16.8.2021.
//

import SwiftUI

struct TCBalance: View {
    var value: MscCurrency
    var body: some View {
        HStack {
            Text(value.amount)
                .foregroundColor(Asset.text600.swiftUIColor)
            Text(value.units).foregroundColor(Asset.text600.swiftUIColor)
            Spacer()
        }
    }
}

// struct TCBalance_Previews: PreviewProvider {
//    static var previews: some View {
//        TCBalance()
//    }
// }
