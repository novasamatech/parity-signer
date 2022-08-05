//
//  HeaderBar.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 14.10.2021.
//

import SwiftUI

struct HeadingOverline: View {
    var text: String
    var body: some View {
        Text(text)
            .foregroundColor(Asset.text600.swiftUIColor)
            .font(Fontstyle.overline.base)
            .tracking(0.5)
            .textCase(.uppercase)
    }
}

struct HeaderBar: View {
    var line1: String
    var line2: String
    var body: some View {
        VStack(alignment: .leading, spacing: 2) {
            HeadingOverline(text: line1)
            Text(line2)
                .foregroundColor(Asset.text400.swiftUIColor)
                .font(Fontstyle.subtitle2.base)
            Divider()
                .padding(.top, 6)
        }
        .font(.caption)
    }
}

// struct HeaderBar_Previews: PreviewProvider {
//    static var previews: some View {
//        HeaderBar()
//    }
// }
