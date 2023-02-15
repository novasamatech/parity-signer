//
//  TCError.swift
//  Polkadot Vault
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
        .foregroundColor(Asset.accentRed300.swiftUIColor)
        .font(PrimaryFont.bodyM.font)
        .padding(Spacing.small)
        .containerBackground(state: .error)
    }
}

struct TCError_Previews: PreviewProvider {
    static var previews: some View {
        TCError(text: "Error body I guess")
    }
}
