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
                Localizable.unknownNetwork.text
                TCNameValueTemplate(
                    name: Localizable.TCName.genesisHash.string,
                    value: content.networkGenesisHash.formattedAsString
                )
                TCNameValueTemplate(name: Localizable.TCName.version.string, value: content.version)
                TCNameValueTemplate(name: Localizable.TCName.txVersion.string, value: content.txVersion)
            }
            Spacer()
        }
    }
}

// struct TCTXSpecPlain_Previews: PreviewProvider {
// static var previews: some View {
// TCTXSpecPlain()
// }
// }
