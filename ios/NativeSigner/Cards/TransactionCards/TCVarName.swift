//
//  TCVarName.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCVarName: View {
    var text: String
    var body: some View {
        TCNameValueTemplate(name: "", value: text)
    }
}

/*
struct TCVarName_Previews: PreviewProvider {
    static var previews: some View {
        TCVarName()
    }
}
*/
