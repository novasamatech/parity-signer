//
//  NetworkCard.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import SwiftUI

struct NetworkCard: View {
    @EnvironmentObject var data: SignerDataModel
    let network: Network?
    var body: some View {
        HStack {
            //TODO: implement png or svg import intercompatible with fontnames
            switch (network?.logo) {
            case "polkadot":
                Text("polkadot").font(Font.custom("Web3-Regular", size: 24))
            case "kusama":
                Text("kusama").font(Font.custom("Web3-Regular", size: 24))
            case "westend":
                Text("westend").font(Font.custom("Web3-Regular", size: 24))
            case "rococo":
                Text("rococo").font(Font.custom("Web3-Regular", size: 24))
            default:
                Text("substrate").font(Font.custom("Web3-Regular", size: 24))
            }
            Text(network?.title ?? "None")
                .font(.headline)
                .foregroundColor(Color("textMainColor"))
        }.foregroundColor(Color(true ? "textMainColor" : "textFadedColor"))
        //.background(Color(data.selectedNetwork == network ? "backgroundActive" : "backgroundCard"))
    }
}

/*
struct NetworkCard_Previews: PreviewProvider {
    static var network = Network.networkData[0]
    static var previews: some View {
        NetworkCard(network: network).previewLayout(.sizeThatFits)
    }
}
*/
