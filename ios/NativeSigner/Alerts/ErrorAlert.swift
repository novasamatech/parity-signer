//
//  ErrorAlert.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 14.12.2021.
//

import SwiftUI

struct ErrorAlert: View {
    @EnvironmentObject var data: SignerDataModel
    let content: MError
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 20).foregroundColor(Color("BgDanger"))
            VStack{
                Text("Error!").font(FBase(style: .h1)).foregroundColor(Color("SignalDanger"))
                Text(content.error).foregroundColor(Color("SignalDanger"))
                Button(action: {
                    data.pushButton(buttonID: .GoBack)
                }) {Text("Ok")}
            }
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
