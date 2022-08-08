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
                            .foregroundColor(Asset.text600.swiftUIColor)
                        Spacer()
                        if !value.docsEnumVariant.isEmpty {
                            Text("?")
                                .foregroundColor(Asset.action400.swiftUIColor)
                        }
                    }
                    if showDoc {
                        Text(
                            AttributedString(fromHexDocs: value.docsEnumVariant) ??
                                "docs parsing error in iOS, please refer to other sources"
                        )
                        .foregroundColor(Asset.text600.swiftUIColor)
                    }
                }
            }
        ).disabled(value.docsEnumVariant.isEmpty)
    }
}

// struct TCEnumVariantName_Previews: PreviewProvider {
// static var previews: some View {
// TCEnumVariantName()
// }
// }
