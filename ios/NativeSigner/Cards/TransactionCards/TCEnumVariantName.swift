//
//  TCEnumVariantName.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCEnumVariantName: View {
    var value: EnumVariantName
    @State private var showDoc = false
    var body: some View {
        Button (action: {
            self.showDoc.toggle()
        }) {
            VStack {
                HStack {
                    Text(value.name)
                        .foregroundColor(Color("textMainColor"))
                    Spacer()
                    if value.docs_enum_variant != "" {
                        Text("?")
                            .foregroundColor(Color("AccentColor"))
                    }
                }
                .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
                if showDoc {
                    Text(String(fromHexDocs: value.docs_enum_variant) ?? "docs parsing error in iOS, please refer to other sources")
                        .foregroundColor(Color("textMainColor"))
                        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
                }
            }
        }.disabled(value.docs_enum_variant == "")
    }
}

/*
 struct TCEnumVariantName_Previews: PreviewProvider {
 static var previews: some View {
 TCEnumVariantName()
 }
 }
 */
