//
//  TCNamedValueCard.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 7.1.2022.
//

import SwiftUI

struct TCNamedValueCard: View {
    let name: String?
    let value: String?

    init(
        name: String? = nil,
        value: String? = nil
    ) {
        self.name = name
        self.value = value
    }

    var body: some View {
        HStack {
            if let name = name {
                Text(name)
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
            }
            if let value = value {
                Text(value)
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
            }
            Spacer()
        }
        .font(Fontstyle.bodyL.base)
    }
}

struct TCNamedValueCard_Previews: PreviewProvider {
    static var previews: some View {
        TCNamedValueCard(name: "Name", value: "value")
    }
}
