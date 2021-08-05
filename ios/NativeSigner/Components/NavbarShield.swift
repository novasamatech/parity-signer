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
            Image(systemName: "shield.fill")
                .imageScale(.large)
                .foregroundColor(.red)
        } else {
            Image(systemName: "shield.fill")
                .imageScale(.large)
                .foregroundColor(.green)
        }
    }
}

struct NavbarShield_Previews: PreviewProvider {
    static var previews: some View {
        NavbarShield().previewLayout(.sizeThatFits)
    }
}
