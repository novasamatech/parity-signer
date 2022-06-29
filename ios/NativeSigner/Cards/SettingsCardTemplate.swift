//
//  SettingsCardTemplate.swift
//  NativeSigner
//
//  Created by Sveta Goldstein on 15.12.2021.
//

import SwiftUI

struct SettingsCardTemplate: View {
    var text: String
    var danger: Bool = false
    var withIcon: Bool = true
    var withBackground: Bool = true

    var body: some View {
        HStack {
            Text(text)
                .font(FBase(style: .body1))
                .foregroundColor(Color(danger ? "SignalDanger" : "Text400"))
            Spacer()
            if withIcon {
                Image(systemName: "chevron.forward")
                    .imageScale(.medium)
                    .foregroundColor(Color("Border400"))
            }
        }
        .padding()
        .background(Color(withBackground ? "Bg200" : ""))
    }
}
