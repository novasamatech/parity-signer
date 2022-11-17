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
                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        Spacer()
                        if !value.docsEnumVariant.isEmpty {
                            Localizable.questionMark.text
                                .foregroundColor(Asset.accentPink300.swiftUIColor)
                        }
                    }
                    if showDoc {
                        Text.markdownWithFallback(value.docsEnumVariant)
                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    }
                }
                .font(Fontstyle.bodyL.base)
            }
        ).disabled(value.docsEnumVariant.isEmpty)
    }
}

struct TCEnumVariantName_Previews: PreviewProvider {
    static var previews: some View {
        TCEnumVariantName(value: MscEnumVariantName(name: "Name", docsEnumVariant: "docsEnumVariant"))
    }
}
