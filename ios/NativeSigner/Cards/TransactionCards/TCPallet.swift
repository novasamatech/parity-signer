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
        TCNameValueTemplate(name: "Pallet", value: text)
    }
}

/*
 struct TCPallet_Previews: PreviewProvider {
 static var previews: some View {
 TCPallet()
 }
 }
 */
