//
//  HistoryCard.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 13.8.2021.
//

import SwiftUI

struct HistoryCard: View {
    var timestamp: String?
    var danger: Bool
    var line1: String
    var line2: String?

    var body: some View {
        HStack {
            VStack(alignment: .leading, spacing: 2) {
                if let timestamp, !timestamp.isEmpty {
                    Text(timestamp)
                        .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                        .font(PrimaryFont.captionM.font)
                }
                Text(line1)
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    .font(PrimaryFont.bodyM.font)
                    .tracking(0.1)
                if let line2 {
                    Text(line2)
                        .foregroundColor(Asset.accentPink300.swiftUIColor)
                        .font(PrimaryFont.captionM.font)
                }
            }
            Spacer()
        }
        .padding(8)
        .cornerRadius(8)
        .background(danger ? Asset.accentRed300.swiftUIColor.opacity(0.3) : Asset.backgroundSecondary.swiftUIColor)
    }
}
