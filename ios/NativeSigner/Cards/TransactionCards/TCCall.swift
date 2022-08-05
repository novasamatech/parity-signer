//
//  TCCall.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCCall: View {
    let value: MscCall
    @State private var showDoc = false
    var body: some View {
        Button(
            action: {
                self.showDoc.toggle()
            },
            label: {
                VStack {
                    HStack {
                        TCNameValueTemplate(name: "Method", value: value.methodName)
                        if value.docs != "" {
                            Text("?")
                                .foregroundColor(Color("Action400"))
                        }
                    }
                    if showDoc {
                        Text(AttributedString(fromHexDocs: value.docs) ??
                             "docs parsing error in iOS, please refer to other sources")
                            .foregroundColor(Color("Text600")).multilineTextAlignment(.leading)
                    }
                }
            }).disabled(value.docs == "")
    }
}

/*
 struct TCCall_Previews: PreviewProvider {
 static var previews: some View {
 TCCall()
 }
 }
 */
