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
    var color: String
    var line1: String
    var line2: String
    
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 4).foregroundColor(Color("backgroundCard"))
            HStack (alignment: .center) {
                Image(systemName: image)
                    .imageScale(.medium)
                    .foregroundColor(Color(color))
                    .frame(width: 26.0)
                    .padding(8)
                VStack (alignment: .leading) {
                    Text(timestamp)
                        .font(.system(size: 13))
                    Text(line1)
                        .foregroundColor(Color(color))
                        .font(.system(size: 13, weight: .bold))
                    Text(line2)
                        .foregroundColor(Color("textFadedColor"))
                        .font(.system(size: 12, weight: .semibold, design: .monospaced))
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
