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
                showDoc.toggle()
            },
            label: {
                VStack {
                    HStack {
                        Text(value.name)
                            .foregroundColor(.textAndIconsPrimary)
                        Spacer()
                        if !value.docsEnumVariant.isEmpty {
                            Image(.questionCircle)
                                .foregroundColor(.textAndIconsDisabled)
                        }
                    }
                    if showDoc {
                        withAnimation {
                            VStack(alignment: .leading) {
                                Text.markdownWithFallback(value.docsEnumVariant)
                                    .foregroundColor(.textAndIconsPrimary)
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

#if DEBUG
    struct TCEnumVariantName_Previews: PreviewProvider {
        static var previews: some View {
            TCEnumVariantName(value: .stub)
        }
    }
#endif
