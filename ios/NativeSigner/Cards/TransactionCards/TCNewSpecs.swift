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
            Text("NEW NETWORK").foregroundColor(Color("textMainColor"))
            VStack(alignment: .leading) {
                HStack {
                    Text("Network name:")
                        .foregroundColor(Color("textFadedColor"))
                    Text(value.title)
                        .foregroundColor(Color("textMainColor"))
                }
                HStack {
                    Text("base58 prefix:")
                        .foregroundColor(Color("textFadedColor"))
                    Text(value.base58prefix)
                        .foregroundColor(Color("textMainColor"))
                }
                HStack {
                    Text("decimals:")
                        .foregroundColor(Color("textFadedColor"))
                    Text(value.decimals)
                        .foregroundColor(Color("textMainColor"))
                }
                HStack {
                    Text("unit:")
                        .foregroundColor(Color("textFadedColor"))
                    Text(value.unit)
                        .foregroundColor(Color("textMainColor"))
                }
                HStack {
                    Text("genesis hash:")
                        .foregroundColor(Color("textFadedColor"))
                    Text(value.genesis_hash)
                        .foregroundColor(Color("textMainColor"))
                }
                HStack {
                    Text("crypto:")
                        .foregroundColor(Color("textFadedColor"))
                    Text(value.encryption)
                        .foregroundColor(Color("textMainColor"))
                }
                HStack {
                    Text("spec name:")
                        .foregroundColor(Color("textFadedColor"))
                    Text(value.name)
                        .foregroundColor(Color("textMainColor"))
                }
                HStack {
                    Text("logo:")
                        .foregroundColor(Color("textFadedColor"))
                    Text(value.logo)
                        .foregroundColor(Color("textMainColor")).font(Font.custom("Web3-Regular", size: 18))
                }
                HStack {
                    Text("default path:")
                        .foregroundColor(Color("textFadedColor"))
                    Text(value.path_id)
                        .foregroundColor(Color("textMainColor"))
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
