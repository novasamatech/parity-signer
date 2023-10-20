//
//  TCNetworkInfo.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 10.1.2022.
//

import SwiftUI

struct TCNetworkInfo: View {
    let content: MscNetworkInfo

    var body: some View {
        HStack {
            NetworkLogoIcon(networkName: content.networkLogo)
            Text(content.networkTitle)
                .font(PrimaryFont.labelM.font)
                .foregroundColor(.textAndIconsPrimary)
        }
        .frame(height: 36)
        .padding(.horizontal)
    }
}

#if DEBUG
    struct TCNetworkInfo_Previews: PreviewProvider {
        static var previews: some View {
            TCNetworkInfo(
                content: .stub
            )
        }
    }
#endif
