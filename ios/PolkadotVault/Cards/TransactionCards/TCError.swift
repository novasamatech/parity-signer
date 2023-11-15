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
        .foregroundColor(.accentRed300)
        .font(PrimaryFont.bodyM.font)
        .padding(Spacing.small)
        .containerBackground(state: .error)
    }
}

#if DEBUG
    struct TCError_Previews: PreviewProvider {
        static var previews: some View {
            TCError(text: "Error body I guess")
        }
    }
#endif
