//
//  TCTXSpec.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCTXSpec: View {
    let value: TxSpec
    var body: some View {
        HStack {
            Spacer()
            VStack {
                Text("network")
                    .foregroundColor(Color("AccentColor"))
                Text(value.network)
                    .foregroundColor(Color("textMainColor"))
            }
            Spacer()
            VStack {
                Text("spec version")
                    .foregroundColor(Color("AccentColor"))
                Text(value.version)
                    .foregroundColor(Color("textMainColor"))
            }
            Spacer()
            VStack {
                Text("tx version")
                    .foregroundColor(Color("AccentColor"))
                Text(value.tx_version)
                    .foregroundColor(Color("textMainColor"))
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
