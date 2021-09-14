//
//  TCID.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCID: View {
    var text: String
    var body: some View {
        HStack {
            //TODO: handle error
            Image(uiImage: UIImage(data: Data(fromHexEncodedString: String(cString: development_test(nil, text)))!)!)
            Text(text)
                .foregroundColor(Color("textMainColor"))
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
