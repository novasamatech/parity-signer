//
//  TCTXSpec.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCTXSpec: View {
    let value: String
    var body: some View {
        HStack {
            Spacer()
            VStack {
                Text("TX version")
                    .foregroundColor(Color("Text400"))
                Text(value)
                    .foregroundColor(Color("Text600"))
            }
            Spacer()
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
    }
}

/*
struct TCTXSpec_Previews: PreviewProvider {
    static var previews: some View {
        TCTXSpec()
    }
}
*/
