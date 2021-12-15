//
//  TCCall.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCCall: View {
    let value: Call
    @State private var showDoc = false
    var body: some View {
        Button (action: {
            self.showDoc.toggle()
        }) {
            VStack {
                HStack {
                    Text("Method").foregroundColor(Color("Text400"))
                    Text(value.method_name)
                        .foregroundColor(Color("Text600"))
                    Spacer()
                    if value.docs != "" {
                        Text("?")
                        .foregroundColor(Color("Action400"))
                    }
                }
                if showDoc {
                    Text(AttributedString(fromHexDocs: value.docs) ?? "docs parsing error in iOS, please refer to other sources")
                        .foregroundColor(Color("Text600"))
                }
            }
        }.disabled(value.docs == "")
    }
}

/*
 struct TCCall_Previews: PreviewProvider {
 static var previews: some View {
 TCCall()
 }
 }
 */
