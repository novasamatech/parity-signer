//
//  ErrorAlert.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 14.12.2021.
//

import SwiftUI

struct ErrorAlert: View {
    @EnvironmentObject var data: SignerDataModel
    let content: String
    var body: some View {
        ZStack {
            Rectangle().foregroundColor(Color("BgDanger")).opacity(0.3).gesture(TapGesture().onEnded {_ in
                data.pushButton(action: .goBack)
            })
            VStack {
                Text("Error!").font(FBase(style: .h1)).foregroundColor(Color("SignalDanger"))
                Text(content).foregroundColor(Color("SignalDanger"))
                Button(
                    action: {
                        data.pushButton(action: .goBack)
                    },
                    label: {Text("Ok")}
                )
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
