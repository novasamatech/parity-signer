//
//  TCEnumVariantName.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCEnumVariantName: View {
    var value: MscEnumVariantName
    @State private var showDoc = false
    var body: some View {
        Button(
            action: {
                self.showDoc.toggle()
            },
            label: {
                VStack {
                    HStack {
                        Text(value.name)
                            .foregroundColor(Color("Text600"))
                        Spacer()
                        if value.docsEnumVariant != "" {
                            Text("?")
                                .foregroundColor(Color("Action400"))
                        }
                    }
                    if showDoc {
                        Text(AttributedString(fromHexDocs: value.docsEnumVariant) ??
                             "docs parsing error in iOS, please refer to other sources")
                            .foregroundColor(Color("Text600"))
                    }
                }
            }).disabled(value.docsEnumVariant == "")
    }
}

/*
 struct TCEnumVariantName_Previews: PreviewProvider {
 static var previews: some View {
 TCEnumVariantName()
 }
 }
 */
