//
//  TCFieldNumber.swift
//  NativeSigner
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
                self.showDoc.toggle()
            },
            label: {
                HStack {
                    Text(value.number)
                        .foregroundColor(Asset.text600.swiftUIColor)
                    Spacer()
                    if (value.docsFieldNumber + value.pathType + value.docsType).isEmpty {
                        Text("?")
                            .foregroundColor(Asset.text400.swiftUIColor)
                    }
                }
                if showDoc {
                    VStack {
                        Text("Path: " + value.pathType)
                        Text(
                            AttributedString(fromHexDocs: value.docsFieldNumber) ??
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
        ).disabled((value.docsFieldNumber + value.pathType + value.docsType).isEmpty)
    }
}

// struct TCFieldNumber_Previews: PreviewProvider {
// static var previews: some View {
// TCFieldNumber()
// }
// }
