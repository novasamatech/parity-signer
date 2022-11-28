//
//  TCError.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCError: View {
    var text: String
    var body: some View {
        HStack {
            Localizable.errorCapitalised.text
            Text(text)
            Spacer()
        }
        .foregroundColor(Asset.accentRed400.swiftUIColor)
        .font(Fontstyle.bodyM.base)
        .padding(Spacing.small)
        .containerBackground(isTinted: true)
    }
}

struct TCError_Previews: PreviewProvider {
    static var previews: some View {
        TCError(text: "Error body I guess")
    }
}
