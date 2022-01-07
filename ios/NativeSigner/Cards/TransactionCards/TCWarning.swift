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
            Text("Warning!")
                .foregroundColor(Color("SignalDanger")).font(FBase(style: .body2))
            Text(text)
                .foregroundColor(Color("SignalDanger")).font(FBase(style: .body2))
            Spacer()
        }.background(Color("BgDanger"))
    }
}

/*
struct TCWarning_Previews: PreviewProvider {
    static var previews: some View {
        TCWarning()
    }
}
*/
