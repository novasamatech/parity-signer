//
//  HistoryCardTemplate.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 6.10.2021.
//

import SwiftUI

struct HistoryCardTemplate: View {
    var image: Image
    var timestamp: String?
    var danger: Bool
    var line1: String
    var line2: String = ""

    var body: some View {
        HStack {
            VStack(alignment: .leading, spacing: 2) {
                if let timestamp = timestamp, !timestamp.isEmpty {
                    Text(timestamp)
                        .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                        .font(PrimaryFont.bodyM.font)
                }
                Text(line1)
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    .font(PrimaryFont.captionM.font)
                    .tracking(0.1)
                Text(line2)
                    .foregroundColor(Asset.accentPink300.swiftUIColor)
                    .font(PrimaryFont.captionM.font)
            }
            Spacer()
            image
                .imageScale(.medium)
                .foregroundColor(danger ? Asset.accentRed400.swiftUIColor : Asset.textAndIconsTertiary.swiftUIColor)
        }
        .padding(8)
        .cornerRadius(8)
        .background(danger ? Asset.accentRed300.swiftUIColor.opacity(0.3) : Asset.backgroundSecondary.swiftUIColor)
    }
}

// struct HistoryCardTemplate_Previews: PreviewProvider {
// static var previews: some View {
// HistoryCardTemplate()
// }
// }
