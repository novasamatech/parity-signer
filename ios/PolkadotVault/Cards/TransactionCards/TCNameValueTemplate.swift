//
//  TCNameValueTemplate.swift
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
                if let name {
                    Text(name)
                        .foregroundColor(.textAndIconsTertiary)
                }
                if let value {
                    Text(value)
                        .foregroundColor(.textAndIconsPrimary)
                }
                Spacer()
            }
            .font(PrimaryFont.bodyL.font)
        } else {
            VStack(alignment: .leading, spacing: Spacing.extraExtraSmall) {
                if let name, name.isEmpty == false {
                    Text(name)
                        .foregroundColor(.textAndIconsTertiary)
                }
                if let value, value.isEmpty == false {
                    Text(value)
                        .foregroundColor(.textAndIconsPrimary)
                }
                HStack {
                    Spacer()
                }
            }
            .font(PrimaryFont.bodyL.font)
        }
    }
}

#if DEBUG
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
#endif
