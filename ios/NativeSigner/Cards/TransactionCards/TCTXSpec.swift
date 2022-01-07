//
//  TCTXSpec.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCTXSpec: View {
    let value: String
    var body: some View {
        TCNameValueTemplate(name: "TX version", value: value)
    }
}

/*
struct TCTXSpec_Previews: PreviewProvider {
    static var previews: some View {
        TCTXSpec()
    }
}
*/
