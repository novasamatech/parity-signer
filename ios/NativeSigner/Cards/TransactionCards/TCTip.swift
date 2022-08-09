//
//  TCTip.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCTip: View {
    var value: MscCurrency
    var body: some View {
        HStack {
            Localizable.tip.text
                .foregroundColor(Asset.text400.swiftUIColor)
            Text(value.amount)
                .foregroundColor(Asset.text600.swiftUIColor)
            Text(value.units)
                .foregroundColor(Asset.text600.swiftUIColor)
            Spacer()
        }
    }
}

// struct TCTip_Previews: PreviewProvider {
//    static var previews: some View {
//        TCTip()
//    }
// }
//
