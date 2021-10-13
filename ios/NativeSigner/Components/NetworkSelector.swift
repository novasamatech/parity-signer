//
//  NetworkList.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import SwiftUI


struct NetworkSelector: View {
    @EnvironmentObject var data: SignerDataModel
    var body: some View {
        Button(action: {
            data.keyManagerModal = .networkManager
        })  {
            HStack {
                NetworkCard(network: data.selectedNetwork)
                Spacer()
                VStack {
                    Text("selected").font(.caption2)
                    Text("network").font(.caption2)
                }
                .padding(.horizontal)
            }
        }
    }
}

/*
 struct NetworkList_Previews: PreviewProvider {
 static var previews: some View {
 NetworkList().previewLayout(.sizeThatFits)
 }
 }
 */
