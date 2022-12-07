//
//  HistoryCard.swift
//  NativeSigner
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
                if let timestamp = timestamp, !timestamp.isEmpty {
                    Text(timestamp)
                        .foregroundColor(Asset.text400.swiftUIColor)
                        .font(Fontstyle.subtitle2.base)
                }
                Text(line1)
                    .foregroundColor(Asset.text600.swiftUIColor)
                    .font(Fontstyle.subtitle1.base)
                    .tracking(0.1)
                if let line2 = line2 {
                    Text(line2)
                        .foregroundColor(Asset.crypto400.swiftUIColor)
                        .font(Fontstyle.body1.crypto)
                }
            }
            Spacer()
        }
        .padding(8)
        .cornerRadius(8)
        .background(danger ? Asset.bgDanger.swiftUIColor : Asset.bg200.swiftUIColor)
    }
}

// struct HistoryCard_Previews: PreviewProvider {
// static var previews: some View {
// HistoryCard()
// }
// }
