//
//  TCNetworkInfo.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 10.1.2022.
//

import SwiftUI

struct TCNetworkInfo: View {
    let content: MscNetworkInfo
    var body: some View {
        HStack {
            NetworkCard(title: content.networkTitle, logo: content.networkLogo)
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
