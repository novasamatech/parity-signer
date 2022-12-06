//
//  TCAuthorPublicKey.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 16.8.2021.
//

import SwiftUI

struct TCAuthorPublicKey: View {
    var value: MVerifierDetails
    var body: some View {
        HStack {
            Identicon(identicon: value.identicon)
            VStack(alignment: .leading) {
                Text(Localizable.TCAuthor.signedWith(value.encryption))
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    .font(Fontstyle.bodyL.base)
                Text(value.publicKey)
                    .font(Fontstyle.captionM.base)
                    .foregroundColor(Asset.accentPink300.swiftUIColor)
            }
            Spacer()
        }
    }
}

struct TCAuthorPublicKey_Previews: PreviewProvider {
    static var previews: some View {
        TCAuthorPublicKey(
            value:
            MVerifierDetails(
                publicKey: PreviewData.publicKey,
                identicon: PreviewData.exampleIdenticon,
                encryption: "sh29919"
            )
        )
    }
}
