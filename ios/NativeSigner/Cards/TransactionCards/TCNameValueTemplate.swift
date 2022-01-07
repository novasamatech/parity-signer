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
            Text(name).foregroundColor(Color("Text400")).font(FBase(style: .body2))
            Text(value).foregroundColor(Color("Text600")).font(FBase(style: .body2))
            Spacer()
        }
    }
}

/*
struct TCNameValueTemplate_Previews: PreviewProvider {
    static var previews: some View {
        TCNameValueTemplate()
    }
}
*/
