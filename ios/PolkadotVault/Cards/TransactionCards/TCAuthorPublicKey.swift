//
//  TCAuthorPublicKey.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 16.8.2021.
//

import SwiftUI

struct TCAuthorPublicKey: View {
    var value: MVerifierDetails

    var body: some View {
        HStack {
            IdenticonView(identicon: value.identicon)
            VStack(alignment: .leading) {
                Text(Localizable.TCAuthor.signedWith(value.encryption))
                    .foregroundColor(.textAndIconsTertiary)
                    .font(PrimaryFont.bodyL.font)
                Text(value.publicKey)
                    .font(PrimaryFont.captionM.font)
                    .foregroundColor(.accentPink300)
            }
            Spacer()
        }
    }
}

#if DEBUG
    struct TCAuthorPublicKey_Previews: PreviewProvider {
        static var previews: some View {
            TCAuthorPublicKey(
                value: .stub
            )
        }
    }
#endif
