//
//  NavbarShield.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 27.7.2021.
//

import SwiftUI
import Network

struct NavbarShield: View {
    @EnvironmentObject var canary: Canary
    var body: some View {
        if canary.dead {
            Image(systemName: "shield.slash")
                .imageScale(.large)
                .foregroundColor(Color("dangerColor"))
        } else {
            Image(systemName: "shield")
                .imageScale(.large)
                .foregroundColor(Color("AccentColor"))
        }
    }
}

/*
struct NavbarShield_Previews: PreviewProvider {
    static var previews: some View {
        NavbarShield().previewLayout(.sizeThatFits)
    }
}*/
