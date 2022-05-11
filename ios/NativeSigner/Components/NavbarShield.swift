//
//  NavbarShield.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 27.7.2021.
//

import SwiftUI
import Network

struct NavbarShield: View {
    @EnvironmentObject var data: SignerDataModel
    var body: some View {
        if data.canaryDead /*bluetooth detector: `|| data.bsDetector.canaryDead`*/ {
            Button(action: {
                data.pushButton(action: .shield)
            }) {
                Image(systemName: "shield.slash")
                    .imageScale(.large)
                    .foregroundColor(Color("SignalDanger"))
            }
        } else {
            if data.alert {
                Button(action: {
                    data.pushButton(action: .shield)
                }) {
                    Image(systemName: "exclamationmark.shield")
                        .imageScale(.large)
                        .foregroundColor(Color("SignalWarning"))
                }
            } else {
                Button(action: {
                    data.pushButton(action: .shield)
                }) {
                    Image(systemName: "lock.shield.fill")
                        .imageScale(.large)
                        .foregroundColor(Color("Crypto400"))
                }
            }
        }
    }
}

/*
 struct NavbarShield_Previews: PreviewProvider {
 static var previews: some View {
 NavbarShield().previewLayout(.sizeThatFits)
 }
 }*/
