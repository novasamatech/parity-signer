//
//  TCID.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCID: View {
    var value: Id
    var body: some View {
        HStack {
            //TODO: handle error
            Image(uiImage: UIImage(data: Data(fromHexEncodedString: value.identicon) ?? Data()) ?? UIImage())
                .resizable(resizingMode: .stretch)
                .frame(width: 28, height: 28)
            Text(value.base58)
                .foregroundColor(Color("Text600"))
            Spacer()
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
    }
}

/*
struct TCID_Previews: PreviewProvider {
    static var previews: some View {
        TCID()
    }
}
*/
