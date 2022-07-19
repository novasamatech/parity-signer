//
//  ErrorAlert.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 14.12.2021.
//

import SwiftUI

struct ErrorAlert: View {
    let pushButton: (Action, String, String) -> Void
    let content: String
    var body: some View {
        ZStack {
            Rectangle().foregroundColor(Color("BgDanger")).opacity(0.3).gesture(TapGesture().onEnded {_ in
                pushButton(.goBack, "", "")
                })
            VStack {
                Text("Error!").font(FBase(style: .h1)).foregroundColor(Color("SignalDanger"))
                Text(content).foregroundColor(Color("SignalDanger"))
                Button("Ok", action: {
                    pushButton(.goBack, "", "")
                })
            }.padding().background(RoundedRectangle(cornerRadius: 20).foregroundColor(Color("BgDanger")))
        }
    }
}

/*
 struct ErrorAlert_Previews: PreviewProvider {
 static var previews: some View {
 ErrorAlert()
 }
 }
 */
