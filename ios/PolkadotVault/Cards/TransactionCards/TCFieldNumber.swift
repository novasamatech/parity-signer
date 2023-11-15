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
                            .foregroundColor(.textAndIconsPrimary)
                        Spacer()
                        if value.displayableValue.isEmpty {
                            Image(.questionCircle)
                                .foregroundColor(.textAndIconsDisabled)
                        }
                    }
                    if showDoc {
                        withAnimation {
                            VStack(alignment: .leading) {
                                Text(Localizable.TCField.path(value.pathType))
                                    .foregroundColor(.accentPink300)
                                Text.markdownWithFallback(value.docsFieldNumber)
                                    .foregroundColor(.textAndIconsPrimary)
                                Text.markdownWithFallback(value.docsType)
                                    .foregroundColor(.textAndIconsPrimary)
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
