//
//  TCAuthorPlain.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 16.8.2021.
//

import SwiftUI

struct TCAuthorPlain: View {
    var value: MscId
    var body: some View {
        HStack {
            Identicon(identicon: value.identicon)
            TCNamedValueCard(name: Localizable.TCName.from.string, value: value.base58)
        }
    }
}

#if DEBUG
    struct TCAuthorPlain_Previews: PreviewProvider {
        static var previews: some View {
            TCAuthorPlain(value: .stub)
        }
    }
#endif
