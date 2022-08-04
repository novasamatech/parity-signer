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
                .foregroundColor(Asset.signalDanger.swiftUIColor).font(Fontstyle.body2.base)
            Text(text)
                .foregroundColor(Asset.signalDanger.swiftUIColor).font(Fontstyle.body2.base)
            Spacer()
        }.background(Asset.bgDanger.swiftUIColor)
    }
}

// struct TCWarning_Previews: PreviewProvider {
//    static var previews: some View {
//        TCWarning()
//    }
// }
