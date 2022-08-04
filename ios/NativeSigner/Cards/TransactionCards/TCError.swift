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
                .foregroundColor(Asset.signalDanger.swiftUIColor).font(Fontstyle.body2.base)
            Text(text)
                .foregroundColor(Asset.signalDanger.swiftUIColor).font(Fontstyle.body2.base)
            Spacer()
        }.background(Asset.bgDanger.swiftUIColor)
    }
}

// struct TCError_Previews: PreviewProvider {
//    static var previews: some View {
//        TCError()
//    }
// }
