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
            Image(uiImage: UIImage(data: Data(fromHexEncodedString: author.identicon)!)!)
            VStack (alignment: .leading) {
                Text("From:")
                    .foregroundColor(Color("Action400"))
                Text(author.seed + author.derivation_path)
                    .foregroundColor(Color("Text600"))
                Text(author.base58)
                    .font(.caption2)
                    .foregroundColor(Color("Text600"))
            }
            Spacer()
        }
    }
}
 /*
struct TCAuthor_Previews: PreviewProvider {
    static var previews: some View {
        TCAuthor()
    }
}*/
