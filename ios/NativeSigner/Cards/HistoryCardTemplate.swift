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
        ZStack {
            RoundedRectangle(cornerRadius: 4).foregroundColor(Color(danger ? "BgDanger" : "Bg200"))
            HStack (alignment: .center) {
                Image(systemName: image)
                    .imageScale(.medium)
                    .foregroundColor(Color(danger ? "SignelDanger" : "Text400"))
                    .frame(width: 26.0)
                    .padding(8)
                VStack (alignment: .leading) {
                    if (timestamp != "") {
                        Text(timestamp)
                            .font(FBase(style: .subtitle2)).foregroundColor(Color("Text400"))
                    }
                    Text(line1)
                        .foregroundColor(Color("Text600"))
                        .font(FBase(style: .subtitle1))
                    Text(line2)
                        .foregroundColor(Color("Crypto400"))
                        .font(FCrypto(style: .body1))
                }
                Spacer()
            }
            .padding(8)
        }
    }
}

/*
 struct HistoryCardTemplate_Previews: PreviewProvider {
 static var previews: some View {
 HistoryCardTemplate()
 }
 }
 */
