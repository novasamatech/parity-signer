//
//  NavbarShield.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 27.7.2021.
//

import SwiftUI

struct NavbarShield: View {
    var body: some View {
        Image(systemName: "shield").imageScale(.large).foregroundColor(/*@START_MENU_TOKEN@*/.green/*@END_MENU_TOKEN@*/)
    }
}

struct NavbarShield_Previews: PreviewProvider {
    static var previews: some View {
        NavbarShield().previewLayout(.sizeThatFits)
    }
}
