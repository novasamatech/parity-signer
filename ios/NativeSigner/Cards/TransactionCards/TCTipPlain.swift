//
//  TCTipPlain.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 7.1.2022.
//

import SwiftUI

struct TCTipPlain: View {
    let content: String
    var body: some View {
        TCNameValueTemplate(name: "Tip", value: content)
    }
}

/*
struct TCTipPlain_Previews: PreviewProvider {
    static var previews: some View {
        TCTipPlain()
    }
}
*/
