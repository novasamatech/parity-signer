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
                            Text("?")
                                .foregroundColor(Asset.action400.swiftUIColor)
                        }
                    }
                    if showDoc {
                        VStack {
                            Text("Path: " + value.pathType).foregroundColor(Asset.text600.swiftUIColor)
                            Text(
                                AttributedString(fromHexDocs: value.docsFieldName) ??
                                    "docs parsing error in iOS, please refer to other sources"
                            )
                            .foregroundColor(Asset.text600.swiftUIColor)
                            Text(
                                AttributedString(fromHexDocs: value.docsType) ??
                                    "docs parsing error in iOS, please refer to other sources"
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

// struct TCFieldName_Previews: PreviewProvider {
// static var previews: some View {
// TCFieldName()
// }
// }
