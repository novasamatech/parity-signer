//
//  TCPallet.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 23.9.2021.
//

import SwiftUI

struct TCPallet: View {
    let text: String
    @State private var showDoc = false
    var body: some View {
        VStack {
            HStack {
                Text("Pallet").foregroundColor(Color("Text400"))
                Text(text)
                    .foregroundColor(Color("Text600"))
                Spacer()
            }
        }
    }
}

/*
 struct TCPallet_Previews: PreviewProvider {
 static var previews: some View {
 TCPallet()
 }
 }
 */
