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
                .foregroundColor(Color("Action400"))
            Text(text)
                .foregroundColor(Color("Text600"))
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
