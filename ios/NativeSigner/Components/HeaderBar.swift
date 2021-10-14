//
//  HeaderBar.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 14.10.2021.
//

import SwiftUI

struct HeaderBar: View {
    var line1: String
    var line2: String
    var body: some View {
        VStack (alignment: .leading) {
            Text(line1).foregroundColor(Color("textMainColor"))
            Text(line2).foregroundColor(Color("textFadedColor"))
            Divider()
        }.padding(.horizontal)
    }
}

/*
struct HeaderBar_Previews: PreviewProvider {
    static var previews: some View {
        HeaderBar()
    }
}
*/
