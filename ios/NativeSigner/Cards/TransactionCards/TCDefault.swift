//
//  TCDefault.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 7.1.2022.
//

import SwiftUI

struct TCDefault: View {
    let content: String
    var body: some View {
        Text(content)
            .foregroundColor(Asset.text600.swiftUIColor)
            .font(Fontstyle.body2.base)
    }
}

// struct TCDefault_Previews: PreviewProvider {
//    static var previews: some View {
//        TCDefault()
//    }
// }
