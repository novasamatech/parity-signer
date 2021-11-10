//
//  TCEraMortalNonce.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 16.8.2021.
//

import SwiftUI

struct TCEraMortal: View {
    var eraMortal: EraMortal
    var body: some View {
        HStack {
            Spacer()
            VStack {
                Text("phase")
                    .foregroundColor(Color("AccentColor"))
                Text(eraMortal.phase)
                    .foregroundColor(Color("textMainColor"))
            }
            Spacer()
            VStack {
                Text("period")
                    .foregroundColor(Color("AccentColor"))
                Text(eraMortal.period)
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
