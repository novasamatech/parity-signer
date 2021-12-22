//
//  TCVarName.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCVarName: View {
    var text: String
    var body: some View {
        HStack {
            Text(text)
                .foregroundColor(Color("Text400"))
            Spacer()
        }
    }
}

/*
struct TCVarName_Previews: PreviewProvider {
    static var previews: some View {
        TCVarName()
    }
}
*/
