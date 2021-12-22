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
    }
}

/*
struct TCWarning_Previews: PreviewProvider {
    static var previews: some View {
        TCWarning()
    }
}
*/
