//
//  TCPathDocs.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 14.9.2021.
//

import SwiftUI

struct TCPathDocs: View {
    var value: PathDocs
    @State private var showDoc = false
    var body: some View {
        Button (action: {
            self.showDoc.toggle()
        }) {
            VStack {
                HStack {
                    Text(value.path.joined(separator: " : "))
                        .foregroundColor(Color("textMainColor"))
                    Spacer()
                    if value.docs != "" {
                        Text("?")
                            .foregroundColor(Color("AccentColor"))
                    }
                }
                .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
                if showDoc {
                    Text(AttributedString(fromHexDocs: value.docs) ?? "docs parsing error in iOS, please refer to other sources")
                        .foregroundColor(Color("textMainColor"))
                        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
                }
            }
        }
    }
}

/*
 struct TCPathDocs_Previews: PreviewProvider {
 static var previews: some View {
 TCPathDocs()
 }
 }
 */
