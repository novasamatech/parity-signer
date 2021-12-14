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
            RoundedRectangle(cornerRadius: 20)
            VStack{
                Text("Error!")
                Text(content.error)
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
