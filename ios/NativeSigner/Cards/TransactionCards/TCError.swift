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
            Text("Error!")
                .foregroundColor(Color("SignalDanger")).font(Fontstyle.body2.base)
            Text(text)
                .foregroundColor(Color("SignalDanger")).font(Fontstyle.body2.base)
            Spacer()
        }.background(Color("BgDanger"))
    }
}

// struct TCError_Previews: PreviewProvider {
//    static var previews: some View {
//        TCError()
//    }
// }
