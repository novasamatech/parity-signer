//
//  TCNamedValueCard.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 7.1.2022.
//

import SwiftUI

struct TCNamedValueCard: View {
    let name: String?
    let value: String?
    let valueInSameLine: Bool

    init(
        name: String? = nil,
        value: String? = nil,
        valueInSameLine: Bool = true
    ) {
        self.name = name
        self.value = value
        self.valueInSameLine = valueInSameLine
    }

    var body: some View {
        if valueInSameLine {
            HStack(alignment: .top, spacing: Spacing.extraSmall) {
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
            .font(PrimaryFont.bodyL.font)
        } else {
            VStack(alignment: .leading, spacing: Spacing.extraExtraSmall) {
                if let name = name, name.isEmpty == false {
                    Text(name)
                        .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                }
                if let value = value, value.isEmpty == false {
                    Text(value)
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                }
                HStack {
                    Spacer()
                }
            }
            .font(PrimaryFont.bodyL.font)
        }
    }
}

struct TCNamedValueCard_Previews: PreviewProvider {
    static var previews: some View {
        TCNamedValueCard(name: "Name", value: "5DCmwXp8XLzSMUyE4uhJMKV4vwvsWqqBYFKJq38CW53VHEVq")
        TCNamedValueCard(name: "Name", value: "value")
        TCNamedValueCard(
            name: "Name",
            value: "5DCmwXp8XLzSMUyE4uhJMKV4vwvsWqqBYFKJq38CW53VHEVq",
            valueInSameLine: false
        )
    }
}
