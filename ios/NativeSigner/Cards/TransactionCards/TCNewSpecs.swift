//
//  TCNewSpecs.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 21.10.2021.
//

import SwiftUI

struct TCNewSpecs: View {
    var value: NewSpecs
    var body: some View {
        VStack {
            Text("NEW NETWORK").foregroundColor(Color("Text600"))
            VStack(alignment: .leading) {
                HStack {
                    Text("Network name:")
                        .foregroundColor(Color("Text400"))
                    Text(value.title)
                        .foregroundColor(Color("Text600"))
                }
                HStack {
                    Text("base58 prefix:")
                        .foregroundColor(Color("Text400"))
                    Text(value.base58prefix)
                        .foregroundColor(Color("Text600"))
                }
                HStack {
                    Text("decimals:")
                        .foregroundColor(Color("Text400"))
                    Text(value.decimals)
                        .foregroundColor(Color("Text600"))
                }
                HStack {
                    Text("unit:")
                        .foregroundColor(Color("Text400"))
                    Text(value.unit)
                        .foregroundColor(Color("Text600"))
                }
                HStack {
                    Text("genesis hash:")
                        .foregroundColor(Color("Text400"))
                    Text(value.genesis_hash)
                        .foregroundColor(Color("Text600"))
                }
                HStack {
                    Text("crypto:")
                        .foregroundColor(Color("Text400"))
                    Text(value.encryption)
                        .foregroundColor(Color("Text600"))
                }
                HStack {
                    Text("spec name:")
                        .foregroundColor(Color("Text400"))
                    Text(value.name)
                        .foregroundColor(Color("Text600"))
                }
                HStack {
                    Text("logo:")
                        .foregroundColor(Color("Text400"))
                    Text(value.logo)
                        .foregroundColor(Color("Text600")).font(Font.custom("Web3-Regular", size: 18))
                }
                HStack {
                    Text("default path:")
                        .foregroundColor(Color("Text400"))
                    Text(value.path_id)
                        .foregroundColor(Color("Text600"))
                }
            }
        }
    }
}
    
    /*
     struct TCNewSpecs_Previews: PreviewProvider {
     static var previews: some View {
     TCNewSpecs()
     }
     }*/
