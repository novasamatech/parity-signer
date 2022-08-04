//
//  TCNameValueTemplate.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 7.1.2022.
//

import SwiftUI

struct TCNameValueTemplate: View {
    let name: String
    let value: String
    var body: some View {
        HStack {
            Text(name).foregroundColor(Asset.text400.swiftUIColor).font(Fontstyle.body2.base)
            Text(value).foregroundColor(Asset.text600.swiftUIColor).font(Fontstyle.body2.base)
            Spacer()
        }
    }
}

// struct TCNameValueTemplate_Previews: PreviewProvider {
//    static var previews: some View {
//        TCNameValueTemplate()
//    }
// }
