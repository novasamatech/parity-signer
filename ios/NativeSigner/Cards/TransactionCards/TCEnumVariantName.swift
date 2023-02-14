//
//  TCEnumVariantName.swift
//  Polkadot Vault
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
                            Asset.questionCircle.swiftUIImage
                                .foregroundColor(Asset.textAndIconsDisabled.swiftUIColor)
                        }
                    }
                    if showDoc {
                        withAnimation {
                            VStack(alignment: .leading) {
                                Text.markdownWithFallback(value.docsEnumVariant)
                                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                                HStack {
                                    Spacer()
                                }
                            }
                            .padding(.horizontal, Spacing.medium)
                            .padding(.vertical, Spacing.small)
                            .strokeContainerBackground()
                        }
                    }
                }
                .font(PrimaryFont.bodyL.font)
            }
        ).disabled(value.docsEnumVariant.isEmpty)
    }
}

struct TCEnumVariantName_Previews: PreviewProvider {
    static var previews: some View {
        TCEnumVariantName(value: MscEnumVariantName(name: "Name", docsEnumVariant: PreviewData.exampleMarkdownDocs))
    }
}
