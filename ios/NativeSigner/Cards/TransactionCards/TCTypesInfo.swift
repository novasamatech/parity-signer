//
//  TCTypesInfo.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCTypesInfo: View {
    var text: String
    var body: some View {
        HStack {
            Text("Types hash:")
                .foregroundColor(Color("AccentColor"))
            Text(text)
                .foregroundColor(Color("textMainColor"))
            Spacer()
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
    }
}

/*
struct TCTypesInfo_Previews: PreviewProvider {
    static var previews: some View {
        TCTypesInfo()
    }
}
*/
