//
//  TCFieldName.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 14.9.2021.
//

import SwiftUI

struct TCFieldName: View {
    var value: FieldName
    @State private var showDoc = false
    var body: some View {
        Button (action: {
            self.showDoc.toggle()
        }) {
            VStack {
                HStack {
                    Text(value.name)
                        .foregroundColor(Color("textMainColor"))
                    Spacer()
                    if value.docs_field_name + value.path_type + value.docs_type != "" {
                        Text("?")
                            .foregroundColor(Color("AccentColor"))
                    }
                }
                .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
                if showDoc {
                    VStack {
                        Text("Path: " + value.path_type)
                        Text(AttributedString(fromHexDocs: value.docs_field_name) ?? "docs parsing error in iOS, please refer to other sources")
                            .foregroundColor(Color("textMainColor"))
                            .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
                        Text(AttributedString(fromHexDocs: value.docs_type) ?? "docs parsing error in iOS, please refer to other sources")
                            .foregroundColor(Color("textMainColor"))
                            .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
                    }
                }
            }
        }
    }
}

/*
 struct TCFieldName_Previews: PreviewProvider {
 static var previews: some View {
 TCFieldName()
 }
 }*/
