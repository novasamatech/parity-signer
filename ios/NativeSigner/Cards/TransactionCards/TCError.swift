//
//  TCError.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCError: View {
    var text: String
    var body: some View {
        HStack {
            Text("Error! ")
                .foregroundColor(Color("textMainColor"))
            Text(text)
                .foregroundColor(Color("textMainColor"))
            Spacer()
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("borderSignalColor")/*@END_MENU_TOKEN@*/)
    }
}

/*
struct TCError_Previews: PreviewProvider {
    static var previews: some View {
        TCError()
    }
}
*/
