//
//  NetworkList.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import SwiftUI


struct NetworkList: View {
    @EnvironmentObject var data: SignerDataModel
    var body: some View {
        Menu {
            ForEach(data.networks, id: \.key) {
                network in
                Button(action: {
                    data.selectNetwork(network: network)
                }) {
                    NetworkCard(network: network)
                }
            }
            .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
        } label: {
            VStack (alignment: .leading) {
                Text("Selected network").font(.footnote)
                NetworkCard(network: data.selectedNetwork)
            }
        }.padding()
    }
}

struct NetworkList_Previews: PreviewProvider {
    static var previews: some View {
        NetworkList().previewLayout(.sizeThatFits)
    }
}
