//
//  NetworkCard.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import SwiftUI

struct NetworkCard: View {
    let network: Network?
    var body: some View {
        HStack {
            Image(systemName: "square")
                .foregroundColor(Color("textMainColor"))
            Text(network?.title ?? "None")
                .font(.headline)
                .fontWeight(.bold)
                .foregroundColor(Color("textMainColor"))
            Spacer()
        }
    }
}

struct NetworkCard_Previews: PreviewProvider {
    static var network = Network.networkData[0]
    static var previews: some View {
        NetworkCard(network: network).previewLayout(.sizeThatFits)
    }
}
