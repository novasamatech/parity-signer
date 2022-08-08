//
//  ErrorAlert.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 14.12.2021.
//

import SwiftUI

struct ErrorAlert: View {
    let navigationRequest: NavigationRequest
    let content: String
    var body: some View {
        ZStack {
            Rectangle().foregroundColor(Asset.bgDanger.swiftUIColor).opacity(0.3).gesture(TapGesture().onEnded { _ in
                navigationRequest(.init(action: .goBack))
            })
            VStack {
                Text("Error!").font(Fontstyle.header1.base).foregroundColor(Asset.signalDanger.swiftUIColor)
                Text(content).foregroundColor(Asset.signalDanger.swiftUIColor)
                Button("Ok", action: {
                    navigationRequest(.init(action: .goBack))
                })
            }.padding().background(RoundedRectangle(cornerRadius: 20).foregroundColor(Asset.bgDanger.swiftUIColor))
        }
    }
}

// struct ErrorAlert_Previews: PreviewProvider {
// static var previews: some View {
// ErrorAlert()
// }
// }
