//
//  TCFieldName.swift
//  NativeSigner
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
                self.showDoc.toggle()
            },
            label: {
                VStack {
                    HStack {
                        Text(value.name)
                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        Spacer()
                        if !(value.docsFieldName + value.pathType + value.docsType).isEmpty {
                            Localizable.questionMark.text
                                .foregroundColor(Asset.accentPink300.swiftUIColor)
                        }
                    }
                    if showDoc {
                        VStack {
                            Text(Localizable.TCField.path(value.pathType))
                                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                            Text.markdownWithFallback(value.docsFieldName, allowsEmptyValue: true)
                                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                            Text.markdownWithFallback(value.docsType, allowsEmptyValue: true)
                                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        }
                    }
                }
                .font(Fontstyle.bodyL.base)
            }
        )
        .disabled((value.docsFieldName + value.pathType + value.docsType).isEmpty)
    }
}

struct TCFieldName_Previews: PreviewProvider {
    static var previews: some View {
        VStack {
            TCFieldName(value: MscFieldName(
                name: "Name",
                docsFieldName: "docsfieldname",
                pathType: "pathType",
                docsType: "docsType"
            ))
            TCFieldName(value: MscFieldName(
                name: "Name",
                docsFieldName: "docsfieldname",
                pathType: "pathType",
                docsType: "docsType"
            ))
        }
        .preferredColorScheme(.dark)
    }
}
