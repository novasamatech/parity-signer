//
//  NetworkCard.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import SwiftUI

struct NetworkCard: View {
    let network: Network
    var body: some View {
        HStack {
            Image(systemName: "square")
                .foregroundColor(Color("textMainColor"))
            Text(network.title)
                .font(.largeTitle)
                .fontWeight(.bold)
                .foregroundColor(Color("textMainColor"))
            Spacer()
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundColor")/*@END_MENU_TOKEN@*/)
    }
}

struct NetworkCard_Previews: PreviewProvider {
    static var network = Network.networkData[0]
    static var previews: some View {
        NetworkCard(network: network).previewLayout(.sizeThatFits)
    }
}
