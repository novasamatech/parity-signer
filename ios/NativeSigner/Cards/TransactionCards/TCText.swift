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
                .foregroundColor(Color("textMainColor"))
            Spacer()
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("AccentColor")/*@END_MENU_TOKEN@*/)
    }
}

/*
struct TCText_Previews: PreviewProvider {
    static var previews: some View {
        TCText()
    }
}
*/
