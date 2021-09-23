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
                    if value.docs != "" {
                        Text("?")
                            .foregroundColor(Color("AccentColor"))
                    }
                }
                .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
                if showDoc {
                    Text(String(decoding: Data(fromHexEncodedString: value.docs) ?? Data(), as: UTF8.self))
                        .foregroundColor(Color("textMainColor"))
                        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
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
