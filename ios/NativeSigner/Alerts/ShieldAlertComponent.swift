//
//  ShieldAlert.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 9.5.2022.
//

import SwiftUI

struct ShieldAlertComponent: View {
    @EnvironmentObject var data: SignerDataModel
    var content: ShieldAlert
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 8)
            switch(content) {
            case .active:
                HeaderBar(line1: "Warning", line2: "content.subheader")
                MenuButtonsStack {
                    BigButton(
                        text: "content.yes",
                        action: {
                            data.pushButton(action: .goForward)
                        }
                    )
                    BigButton(
                        text: "content.no",
                        action: {
                            data.pushButton(action: .goBack)
                        }
                    )
                }
            case .past:
                HeaderBar(line1: "Ok", line2: "content.subheader")
                MenuButtonsStack {
                    BigButton(
                        text: "content.no",
                        action: {
                            data.pushButton(action: .goBack)
                        }
                    )
                }
            }
            
        }
    }
}

/*
struct ShieldAlert_Previews: PreviewProvider {
    static var previews: some View {
        ShieldAlert()
    }
}
*/
