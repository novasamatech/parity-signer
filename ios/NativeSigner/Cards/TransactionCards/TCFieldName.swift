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
                            .foregroundColor(Color("Text600"))
                        Spacer()
                        if value.docsFieldName + value.pathType + value.docsType != "" {
                            Text("?")
                                .foregroundColor(Color("Action400"))
                        }
                    }
                    if showDoc {
                        VStack {
                            Text("Path: " + value.pathType).foregroundColor(Color("Text600"))
                            Text(AttributedString(fromHexDocs: value.docsFieldName) ??
                                 "docs parsing error in iOS, please refer to other sources")
                                .foregroundColor(Color("Text600"))
                            Text(AttributedString(fromHexDocs: value.docsType) ??
                                 "docs parsing error in iOS, please refer to other sources")
                                .foregroundColor(Color("Text600"))
                        }
                    }
                }
            })
            .disabled(value.docsFieldName + value.pathType + value.docsType == "")
    }
}

/*
 struct TCFieldName_Previews: PreviewProvider {
 static var previews: some View {
 TCFieldName()
 }
 }*/
