//
//  TCNameVersion.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 10.11.2021.
//

import SwiftUI

struct TCNameVersion: View {
    let value: NameVersion
    var body: some View {
        HStack {
            Spacer()
            VStack {
                Text(value.name)
                    .foregroundColor(Color("Text400"))
                Text(value.version)
                    .foregroundColor(Color("Text600"))
            }
            Spacer()
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
    }
}

/*
struct TCNameVersion_Previews: PreviewProvider {
    static var previews: some View {
        TCNameVersion()
    }
}
*/
