//
//  TCTypesInfo.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCTypesInfo: View {
    var content: MTypesInfo
    var body: some View {
        HStack {
            if let identicon = content.typesIdPic {
                Identicon(identicon: identicon)
            }
            TCNamedValueCard(
                name: Localizable.TCName.typesHash.string,
                value: content.typesHash ?? ""
            )
        }
    }
}

#if DEBUG
    struct TCTypesInfo_Previews: PreviewProvider {
        static var previews: some View {
            VStack {
                TCTypesInfo(
                    content: .stub
                )
            }
        }
    }
#endif
