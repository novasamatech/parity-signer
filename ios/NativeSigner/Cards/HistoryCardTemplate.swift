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
                if timestamp != "" {
                    Text(timestamp)
                        .foregroundColor(Color("Text400"))
                        .font(FBase(style: .subtitle2))
                }
                Text(line1)
                    .foregroundColor(Color("Text600"))
                    .font(FBase(style: .subtitle1))
                    .tracking(0.1)
                Text(line2)
                    .foregroundColor(Color("Crypto400"))
                    .font(FCrypto(style: .body1))
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

/*
 struct HistoryCardTemplate_Previews: PreviewProvider {
 static var previews: some View {
 HistoryCardTemplate()
 }
 }
 */
