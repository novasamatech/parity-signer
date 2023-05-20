//
//  TCFieldNumber.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 14.9.2021.
//

import SwiftUI

struct TCFieldNumber: View {
    var value: MscFieldNumber
    @State private var showDoc = false
    var body: some View {
        Button(
            action: {
                showDoc.toggle()
            },
            label: {
                VStack {
                    HStack {
                        Text(value.number)
                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        Spacer()
                        if value.displayableValue.isEmpty {
                            Asset.questionCircle.swiftUIImage
                                .foregroundColor(Asset.textAndIconsDisabled.swiftUIColor)
                        }
                    }
                    if showDoc {
                        withAnimation {
                            VStack(alignment: .leading) {
                                Text(Localizable.TCField.path(value.pathType))
                                    .foregroundColor(Asset.accentPink300.swiftUIColor)
                                Text.markdownWithFallback(value.docsFieldNumber)
                                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                                Text.markdownWithFallback(value.docsType)
                                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                            }
                            .padding(.horizontal, Spacing.medium)
                            .padding(.vertical, Spacing.small)
                            .strokeContainerBackground()
                        }
                    }
                }
                .font(PrimaryFont.bodyL.font)
            }
        ).disabled(value.displayableValue.isEmpty)
    }
}

private extension MscFieldNumber {
    var displayableValue: String {
        [docsFieldNumber, pathType, docsType].joined()
    }
}

#if DEBUG
    struct TCFieldNumber_Previews: PreviewProvider {
        static var previews: some View {
            TCFieldNumber(value: .stub)
        }
    }
#endif
