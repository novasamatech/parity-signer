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
    var fancy: Bool = false
    var body: some View {
        ZStack {
            if fancy {
                RoundedRectangle(cornerRadius: 4)
                    .foregroundColor(Color("Bg200"))
                    .frame(height: 47)
            }
            HStack {
                NetworkLogo(logo: logo)
                Text(title).font(FBase(style: .h3))
                if fancy {Spacer()}
            }
            .foregroundColor(Color("Text600"))
            .frame(height: 36)
            .padding(.horizontal)
        }
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
