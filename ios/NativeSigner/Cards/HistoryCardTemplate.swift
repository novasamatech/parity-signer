//
//  HistoryCardTemplate.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 6.10.2021.
//

import SwiftUI

struct HistoryCardTemplate: View {
    var image: String
    var timestamp: String
    var danger: Bool
    var line1: String
    var line2: String

    var body: some View {
        HStack {
            VStack(alignment: .leading, spacing: 2) {
                if !timestamp.isEmpty {
                    Text(timestamp)
                        .foregroundColor(Color("Text400"))
                        .font(Fontstyle.subtitle2.base)
                }
                Text(line1)
                    .foregroundColor(Color("Text600"))
                    .font(Fontstyle.subtitle1.base)
                    .tracking(0.1)
                Text(line2)
                    .foregroundColor(Color("Crypto400"))
                    .font(Fontstyle.body1.crypto)
            }
            Spacer()
            Image(systemName: image)
                .imageScale(.medium)
                .foregroundColor(Color(danger ? "SignalDanger" : "Text400"))
        }
        .padding(8)
        .cornerRadius(8)
        .background(Color(danger ? "BgDanger" : "Bg200"))
    }
}

// struct HistoryCardTemplate_Previews: PreviewProvider {
// static var previews: some View {
// HistoryCardTemplate()
// }
// }
