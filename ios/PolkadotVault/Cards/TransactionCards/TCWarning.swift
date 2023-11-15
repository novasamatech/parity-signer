//
//  TCWarning.swift
//  Polkadot Vault
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
        .foregroundColor(.accentRed400)
        .font(PrimaryFont.bodyM.font)
        .padding(Spacing.small)
        .containerBackground(state: .error)
    }
}

#if DEBUG
    struct TCWarning_Previews: PreviewProvider {
        static var previews: some View {
            TCWarning(text: "Content of the warning")
        }
    }
#endif
