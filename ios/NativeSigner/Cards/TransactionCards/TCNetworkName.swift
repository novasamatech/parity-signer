//
//  TCNetworkName.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 7.1.2022.
//

import SwiftUI

struct TCNetworkName: View {
    let content: String
    var body: some View {
        TCNameValueTemplate(name: "Network name", value: content)
    }
}

/*
struct TCNetworkName_Previews: PreviewProvider {
    static var previews: some View {
        TCNetworkName()
    }
}
*/
