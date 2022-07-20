//
//  NetworkLogo.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 7.1.2022.
//

// This should eventually hold logic to handle png network logos.
import SwiftUI

struct NetworkLogo: View {
    let logo: String
    var body: some View {
        Text(logo)
            .foregroundColor(Color("Text600")).font(FWeb3(style: .h4)).frame(width: 36, height: 36)
    }
}

/*
struct NetworkLogo_Previews: PreviewProvider {
    static var previews: some View {
        NetworkLogo()
    }
}
*/
