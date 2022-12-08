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
                        .foregroundColor(Asset.text400.swiftUIColor)
                        .font(Fontstyle.subtitle2.base)
                }
                Text(line1)
                    .foregroundColor(Asset.text600.swiftUIColor)
                    .font(Fontstyle.subtitle1.base)
                    .tracking(0.1)
                Text(line2)
                    .foregroundColor(Asset.crypto400.swiftUIColor)
                    .font(Fontstyle.body1.crypto)
            }
            Spacer()
            image
                .imageScale(.medium)
                .foregroundColor(danger ? Asset.signalDanger.swiftUIColor : Asset.text400.swiftUIColor)
        }
        .padding(8)
        .cornerRadius(8)
        .background(danger ? Asset.bgDanger.swiftUIColor : Asset.bg200.swiftUIColor)
    }
}

// struct HistoryCardTemplate_Previews: PreviewProvider {
// static var previews: some View {
// HistoryCardTemplate()
// }
// }
