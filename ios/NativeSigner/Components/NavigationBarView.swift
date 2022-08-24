//
//  NavigationBarView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 22/08/2022.
//

import SwiftUI

/// UI component that mimics system `NavigationView` and should be used as `NavigationBar` equivalent in `UIKit`
///
/// As we can't switch to `NavigationView` just yet, this should us in the meantime
struct NavigationBarView: View {
    private let title: String

    init(title: String) {
        self.title = title
    }

    var body: some View {
        HStack(alignment: .bottom) {
            Text(title)
                .font(Fontstyle.titleS.base)
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
        }
        .frame(height: 64)
        .background(Asset.backgroundSolidSystem.swiftUIColor)
    }
}
