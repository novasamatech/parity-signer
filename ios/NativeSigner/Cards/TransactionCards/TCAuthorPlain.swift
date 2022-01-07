//
//  TCAuthorPlain.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 16.8.2021.
//

import SwiftUI

struct TCAuthorPlain: View {
    var value: AuthorPlain
    var body: some View {
        HStack {
            Identicon(identicon: value.identicon)
            TCNameValueTemplate(name: "From", value: value.base58)
        }
    }
}

/*
 struct TCAuthorPlain_Previews: PreviewProvider {
 static var previews: some View {
 TCAuthorPlain(author: AuthorPlain(base58: "111"))
 }
 }
 */
