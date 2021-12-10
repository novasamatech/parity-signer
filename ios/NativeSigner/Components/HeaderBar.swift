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
            Text(line1).foregroundColor(Color("Text600")).font(FBase(style: .overline))
            Text(line2).foregroundColor(Color("Text400")).font(FBase(style: .subtitle2))
            Divider()
        }.padding(.horizontal).font(.caption)
    }
}

/*
struct HeaderBar_Previews: PreviewProvider {
    static var previews: some View {
        HeaderBar()
    }
}
*/
