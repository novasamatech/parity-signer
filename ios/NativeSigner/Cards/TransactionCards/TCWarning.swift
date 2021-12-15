//
//  TCWarning.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCWarning: View {
    let text: String
    var body: some View {
        HStack {
            Text("Warning! ")
                .foregroundColor(Color("SignalDanger"))
            Text(text)
                .foregroundColor(Color("SignalDanger"))
            Spacer()
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("AccentColor")/*@END_MENU_TOKEN@*/)
    }
}

/*
struct TCWarning_Previews: PreviewProvider {
    static var previews: some View {
        TCWarning()
    }
}
*/
