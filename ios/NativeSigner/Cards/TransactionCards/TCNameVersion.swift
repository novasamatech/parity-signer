//
//  TCNameVersion.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 10.11.2021.
//

import SwiftUI

struct TCNameVersion: View {
    let value: NameVersion
    var body: some View {
        TCNameValueTemplate(name: value.name, value: value.version)
    }
}

/*
struct TCNameVersion_Previews: PreviewProvider {
    static var previews: some View {
        TCNameVersion()
    }
}
*/
