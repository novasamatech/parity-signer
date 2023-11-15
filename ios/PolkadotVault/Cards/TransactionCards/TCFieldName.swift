//
//  TCFieldName.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 14.9.2021.
//

import SwiftUI

struct TCFieldName: View {
    var value: MscFieldName
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
                        if hasDetails {
                            Image(.questionCircle)
                                .foregroundColor(.textAndIconsDisabled)
                        }
                    }
                    if showDoc {
                        VStack(alignment: .leading) {
                            Text(Localizable.TCField.path(value.pathType))
                                .foregroundColor(.textAndIconsPrimary)
                            Text.markdownWithFallback(value.docsFieldName, allowsEmptyValue: true)
                                .foregroundColor(.textAndIconsPrimary)
                            Text.markdownWithFallback(value.docsType, allowsEmptyValue: true)
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
                .font(PrimaryFont.bodyL.font)
            }
        )
        .disabled(!hasDetails)
    }

    private var hasDetails: Bool {
        !(value.docsFieldName + value.pathType + value.docsType).isEmpty
    }
}

#if DEBUG
    struct TCFieldName_Previews: PreviewProvider {
        static var previews: some View {
            VStack {
                TCFieldName(value: MscFieldName(
                    name: "Namefdsfds",
                    docsFieldName: "docsfieldname",
                    pathType: "pathType",
                    docsType: "docsType"
                ))
            }
            .preferredColorScheme(.dark)
        }
    }
#endif
