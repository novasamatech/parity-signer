//
//  TCID.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCID: View {
    var value: MscId
    var body: some View {
        HStack {
            Identicon(identicon: value.identicon)
            Text(value.base58)
                .foregroundColor(Asset.text600.swiftUIColor).font(Fontstyle.bodyL.crypto)
            Spacer()
        }
    }
}

struct TCID_Previews: PreviewProvider {
    static var previews: some View {
        TCID(
            value: MscId(
                base58: "5F3sa2TJAWMqDhXG6jhV4N8ko9SxwGy8TpaNS1repo5EYjQX",
                identicon: PreviewData.exampleIdenticon
            )
        )
    }
}
