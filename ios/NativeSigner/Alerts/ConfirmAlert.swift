//
//  ConfirmAlert.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 14.12.2021.
//

import SwiftUI

struct ConfirmAlert: View {
    let navigationRequest: NavigationRequest
    let content: String
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 8)
            HeaderBar(line1: content, line2: "content.subheader")
            MenuButtonsStack {
                BigButton(
                    text: "content.yes",
                    action: {
                        navigationRequest(.init(action: .goForward))
                    }
                )
                BigButton(
                    text: "content.no",
                    action: {
                        navigationRequest(.init(action: .goBack))
                    }
                )
            }
        }
    }
}

// struct ConfirmAlert_Previews: PreviewProvider {
//    static var previews: some View {
//        ConfirmAlert()
//    }
// }
