//
//  TCCall.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCCall: View {
    let value: MscCall
    @State private var showDoc = false

    var body: some View {
        Button(
            action: {
                showDoc.toggle()
            },
            label: {
                VStack {
                    HStack {
                        TCNamedValueCard(name: Localizable.TCName.method.string, value: value.methodName)
                        if !value.docs.isEmpty {
                            Asset.questionCircle.swiftUIImage
                                .foregroundColor(Asset.textAndIconsDisabled.swiftUIColor)
                        }
                    }
                    if showDoc {
                        withAnimation {
                            VStack(alignment: .leading) {
                                Text.markdownWithFallback(value.docs)
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
        )
        .disabled(value.docs.isEmpty)
    }
}

#if DEBUG
    struct TCCall_Previews: PreviewProvider {
        static var previews: some View {
            TCCall(value: .stub)
        }
    }
#endif
