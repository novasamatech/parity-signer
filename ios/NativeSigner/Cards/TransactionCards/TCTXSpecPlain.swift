//
//  TCTXSpecPlain.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 7.1.2022.
//

import SwiftUI

struct TCTXSpecPlain: View {
    let content: MscTxSpecPlain
    var body: some View {
        HStack {
            VStack {
                Text("Unknown network")
                TCNameValueTemplate(
                    name: "Genesis hash",
                    value: content.networkGenesisHash.map {String(format: "%02X", $0)}.joined()
                )
                TCNameValueTemplate(name: "Version", value: content.version)
                TCNameValueTemplate(name: "Tx version", value: content.txVersion)
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
