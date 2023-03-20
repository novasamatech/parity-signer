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
                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        Spacer()
                        if hasDetails {
                            Asset.questionCircle.swiftUIImage
                                .foregroundColor(Asset.textAndIconsDisabled.swiftUIColor)
                        }
                    }
                    if showDoc {
                        VStack(alignment: .leading) {
                            Text(Localizable.TCField.path(value.pathType))
                                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                            Text.markdownWithFallback(value.docsFieldName, allowsEmptyValue: true)
                                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                            Text.markdownWithFallback(value.docsType, allowsEmptyValue: true)
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
                .font(PrimaryFont.bodyL.font)
            }
        )
        .disabled(!hasDetails)
    }

    private var hasDetails: Bool {
        !(value.docsFieldName + value.pathType + value.docsType).isEmpty
    }
}

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
