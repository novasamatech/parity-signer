//
//  TCAuthor.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 16.8.2021.
//

import SwiftUI

struct TCAuthor: View {
    var author: Author
    var body: some View {
        HStack {
            Image(uiImage: UIImage(data: Data(fromHexEncodedString: String(cString: base58_identicon(nil, author.base58, 32)))!)!)
            VStack (alignment: .leading) {
                Text("From:")
                    .foregroundColor(Color("AccentColor"))
                Text(author.seed + author.derivation_path)
                    .foregroundColor(Color("textMainColor"))
                Text(author.base58)
                    .font(.caption2)
                    .foregroundColor(Color("textMainColor"))
            }
            Spacer()
        }
        .background(/*@START_MENU_TOKEN@*//*@PLACEHOLDER=View@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
    }
}
 /*
struct TCAuthor_Previews: PreviewProvider {
    static var previews: some View {
        TCAuthor()
    }
}*/
