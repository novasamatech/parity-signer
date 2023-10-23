//
//  TCID.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCID: View {
    var value: MscId
    var body: some View {
        HStack {
            Text(value.base58)
                .foregroundColor(.textAndIconsPrimary)
                .font(PrimaryFont.bodyL.font)
            Spacer()
        }
    }
}

#if DEBUG
    struct TCID_Previews: PreviewProvider {
        static var previews: some View {
            TCID(
                value: MscId(
                    base58: "5F3sa2TJAWMqDhXG6jhV4N8ko9SxwGy8TpaNS1repo5EYjQX",
                    identicon: .stubIdenticon
                )
            )
        }
    }
#endif
