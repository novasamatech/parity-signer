//
//  NetworkCard.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import SwiftUI

struct NetworkCard: View {
    @EnvironmentObject var data: SignerDataModel
    let title: String
    let logo: String
    var body: some View {
        //TODO: implement png or svg import intercompatible with fontnames
        HStack {
            Text(logo).font(FWeb3(style: .h4))
            Text(title).font(FBase(style: .h3))
            Spacer()
        }
        .foregroundColor(Color("Text600"))
        .frame(height: 36)
        .padding(.horizontal)
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
