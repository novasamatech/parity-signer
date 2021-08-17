//
//  TCEraImmortalNonce.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 16.8.2021.
//

import SwiftUI

struct TCEraImmortalNonce: View {
    var eraImmortalNonce: EraImmortalNonce
    var body: some View {
        HStack {
            Spacer()
            VStack {
                Text("nonce")
                    .foregroundColor(Color("AccentColor"))
                Text(eraImmortalNonce.nonce)
                    .foregroundColor(Color("textMainColor"))
            }
            Spacer()
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
    }
}

/*
struct TCEraImmortalNonce_Previews: PreviewProvider {
    static var previews: some View {
        TCEraImmortalNonce()
    }
}
*/
