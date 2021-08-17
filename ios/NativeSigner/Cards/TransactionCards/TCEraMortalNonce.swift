//
//  TCEraMortalNonce.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 16.8.2021.
//

import SwiftUI

struct TCEraMortalNonce: View {
    var eraMortalNonce: EraMortalNonce
    var body: some View {
        HStack {
            Spacer()
            VStack {
                Text("phase")
                    .foregroundColor(Color("AccentColor"))
                Text(eraMortalNonce.phase)
                    .foregroundColor(Color("textMainColor"))
            }
            Spacer()
            VStack {
                Text("period")
                    .foregroundColor(Color("AccentColor"))
                Text(eraMortalNonce.period)
                    .foregroundColor(Color("textMainColor"))
            }
            Spacer()
            VStack {
                Text("nonce")
                    .foregroundColor(Color("AccentColor"))
                Text(eraMortalNonce.nonce)
                    .foregroundColor(Color("textMainColor"))
            }
            Spacer()
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
    }
}

/*
struct TCEraMortalNonce_Previews: PreviewProvider {
    static var previews: some View {
        TCEraMortalNonce()
    }
}
*/
