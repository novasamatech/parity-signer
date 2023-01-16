//
//  HeaderBar.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 14.10.2021.
//

import SwiftUI

struct HeaderBar: View {
    var line1: LocalizedStringKey
    var line2: LocalizedStringKey
    var body: some View {
        VStack(alignment: .leading, spacing: 2) {
            Text(line1)
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .font(PrimaryFont.labelS.font)
                .tracking(0.5)
                .textCase(.uppercase)
            Text(line2)
                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                .font(PrimaryFont.bodyM.font)
            Divider()
                .padding(.top, 6)
        }
        .font(PrimaryFont.captionM.font)
    }
}

// struct HeaderBar_Previews: PreviewProvider {
//    static var previews: some View {
//        HeaderBar()
//    }
// }
