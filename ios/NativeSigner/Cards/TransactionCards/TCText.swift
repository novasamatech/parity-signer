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
            Text(AttributedString(fromHexDocs: text) ?? AttributedString(text))
                .foregroundColor(Color("Text600")).font(FBase(style: .body2))
            Spacer()
        }
    }
}

/*
struct TCText_Previews: PreviewProvider {
    static var previews: some View {
        TCText()
    }
}
*/
