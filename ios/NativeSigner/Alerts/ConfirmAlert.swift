//
//  ConfirmAlert.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 14.12.2021.
//

import SwiftUI

struct ConfirmAlert: View {
    let pushButton: (Action, String, String) -> Void
    let content: String
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 8)
            HeaderBar(line1: content, line2: "content.subheader")
            MenuButtonsStack {
                BigButton(
                    text: "content.yes",
                    action: {
                        pushButton(.goForward, "", "")
                    }
                )
                BigButton(
                    text: "content.no",
                    action: {
                        pushButton(.goBack, "", "")
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
