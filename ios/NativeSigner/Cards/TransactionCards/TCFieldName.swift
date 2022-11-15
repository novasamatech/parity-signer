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
                            .foregroundColor(Asset.text600.swiftUIColor)
                        Spacer()
                        if (value.docsFieldName + value.pathType + value.docsType).isEmpty {
                            Localizable.questionMark.text
                                .foregroundColor(Asset.action400.swiftUIColor)
                        }
                    }
                    if showDoc {
                        VStack {
                            Text(Localizable.TCField.path(value.pathType))
                                .foregroundColor(Asset.text600.swiftUIColor)
                            Text(
                                AttributedString(fromHexDocs: value.docsFieldName) ??
                                    AttributedString(Localizable.Error.docsParsing.string)
                            )
                            .foregroundColor(Asset.text600.swiftUIColor)
                            Text(
                                AttributedString(fromHexDocs: value.docsType) ??
                                    AttributedString(Localizable.Error.docsParsing.string)
                            )
                            .foregroundColor(Asset.text600.swiftUIColor)
                        }
                    }
                }
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
