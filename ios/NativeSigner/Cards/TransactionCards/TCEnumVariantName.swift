//
//  TCEnumVariantName.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCEnumVariantName: View {
    var text: String
    var body: some View {
        HStack {
            Text(text)
                .foregroundColor(Color("textMainColor"))
            Spacer()
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
    }
}

/*
struct TCEnumVariantName_Previews: PreviewProvider {
    static var previews: some View {
        TCEnumVariantName()
    }
}
*/
