//
//  TCText.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 10.11.2021.
//

import SwiftUI

struct TCText: View {
    let text: String
    var body: some View {
        HStack {
            Text(AttributedString.build(fromDocs: text) ?? AttributedString(text))
                .foregroundColor(.textAndIconsPrimary).font(PrimaryFont.bodyL.font)
            Spacer()
        }
    }
}

#if DEBUG
    struct TCText_Previews: PreviewProvider {
        static var previews: some View {
            TCText(text: Stubs.stubMarkdownDocs)
        }
    }
#endif
