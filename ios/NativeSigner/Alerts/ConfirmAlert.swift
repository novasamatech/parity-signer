//
//  ConfirmAlert.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 14.12.2021.
//

import SwiftUI

struct ConfirmAlert: View {
    @EnvironmentObject var data: SignerDataModel
    let content: MConfirm
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 8)
            HeaderBar(line1: content.header, line2: content.subheader)
            MenuButtonsStack {
                BigButton(
                    text: content.yes,
                    action: {
                        data.pushButton(action: .goForward)
                    }
                )
                BigButton(
                    text: content.no,
                    action: {
                        data.pushButton(action: .goBack)
                    }
                )
            }
        }
    }
}

/*
struct ConfirmAlert_Previews: PreviewProvider {
    static var previews: some View {
        ConfirmAlert()
    }
}
*/
