//
//  TCText.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 10.11.2021.
//

import SwiftUI

struct TCText: View {
    let text: String
    var body: some View {
        HStack {
            Text(AttributedString.build(fromHexDocs: text) ?? AttributedString(text))
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor).font(Fontstyle.bodyL.base)
            Spacer()
        }
    }
}

struct TCText_Previews: PreviewProvider {
    static var previews: some View {
        TCText(text: "body text possible markdown support")
    }
}
