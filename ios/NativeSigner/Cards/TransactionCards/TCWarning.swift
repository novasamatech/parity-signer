//
//  TCWarning.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCWarning: View {
    let text: String
    var body: some View {
        HStack {
            Localizable.warning.text
            Text(text)
            Spacer()
        }
        .foregroundColor(Asset.accentRed400.swiftUIColor)
        .font(Fontstyle.bodyM.base)
        .padding(Spacing.small)
        .containerBackground(isTinted: true)
    }
}

struct TCWarning_Previews: PreviewProvider {
    static var previews: some View {
        TCWarning(text: "Content of the warning")
    }
}
