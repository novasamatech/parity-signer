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
        HStack (alignment: .center) {
            Image(systemName: image).imageScale(.large).foregroundColor(Color(color)).frame(width: 26.0).padding()
            VStack (alignment: .leading) {
                Text(timestamp)
                Text(line1)
                    .foregroundColor(Color(color))
                Text(line2).foregroundColor(Color("textFadedColor"))
            }
            Spacer()
        }
        .padding(8)
        .background(Color("backgroundCard"))
    }
}

/*
struct HistoryCardTemplate_Previews: PreviewProvider {
    static var previews: some View {
        HistoryCardTemplate()
    }
}
*/
