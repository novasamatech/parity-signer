//
//  TCNetworkInfo.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 10.1.2022.
//

import SwiftUI

struct TCNetworkInfo: View {
    let content: NetworkInfo
    var body: some View {
        HStack {
            NetworkCard(title: content.network_title, logo: content.network_logo)
        }
    }
}

/*
struct TCNetworkInfo_Previews: PreviewProvider {
    static var previews: some View {
        TCNetworkInfo()
    }
}
*/
