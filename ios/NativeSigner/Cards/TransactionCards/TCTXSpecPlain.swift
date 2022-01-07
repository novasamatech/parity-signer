//
//  TCTXSpecPlain.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 7.1.2022.
//

import SwiftUI

struct TCTXSpecPlain: View {
    let content: TxSpecPlain
    var body: some View {
        HStack{
            VStack{
                Text("Unknown network")
                TCNameValueTemplate(name: "Genesis hash", value: content.network_genesis_hash)
                TCNameValueTemplate(name: "Version", value: content.version)
                TCNameValueTemplate(name: "Tx version", value: content.tx_version)
            }
            Spacer()
        }
    }
}

/*
struct TCTXSpecPlain_Previews: PreviewProvider {
    static var previews: some View {
        TCTXSpecPlain()
    }
}
*/
