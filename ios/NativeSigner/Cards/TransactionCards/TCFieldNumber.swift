//
//  TCFieldNumber.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 14.9.2021.
//

import SwiftUI

struct TCFieldNumber: View {
    var value: FieldNumber
    @State private var showDoc = false
    var body: some View {
        Button (action: {
            self.showDoc.toggle()
        }) {
            HStack {
                Text(value.number)
                    .foregroundColor(Color("Text600"))
                Spacer()
                if value.docs_field_number + value.path_type + value.docs_type != "" {
                    Text("?")
                        .foregroundColor(Color("Text400"))
                }
            }
            if showDoc {
                VStack {
                    Text("Path: " + value.path_type)
                    Text(AttributedString(fromHexDocs: value.docs_field_number) ?? "docs parsing error in iOS, please refer to other sources")
                        .foregroundColor(Color("Text600"))
                    Text(AttributedString(fromHexDocs: value.docs_type) ?? "docs parsing error in iOS, please refer to other sources")
                        .foregroundColor(Color("Text600"))
                }
            }
        }.disabled(value.docs_field_number + value.path_type + value.docs_type == "")
    }
}

/*
 struct TCFieldNumber_Previews: PreviewProvider {
 static var previews: some View {
 TCFieldNumber()
 }
 }*/
