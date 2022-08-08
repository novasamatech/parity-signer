//
//  AuthenticatedScreenContainer.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 05/08/2022.
//

import SwiftUI

struct AuthenticatedScreenContainer: View {
    @ObservedObject var data: SignerDataModel
    @ObservedObject var navigation: NavigationCoordinator
    @GestureState private var dragOffset = CGSize.zero

    var body: some View {
        VStack(spacing: 0) {
            HeaderViewContainer(data: data, navigation: navigation)
            ZStack {
                VStack(spacing: 0) {
                    ScreenSelectorView(data: data, navigation: navigation)
                    Spacer()
                }
                ModalSelectorView(data: data, navigation: navigation)
                AlertSelectorView(data: data, navigation: navigation)
            }
            .gesture(
                DragGesture().updating($dragOffset, body: { value, _, _ in
                    if value.startLocation.x < 20, value.translation.width > 100 {
                        navigation.perform(navigation: .init(action: .goBack))
                    }
                })
            )
            // Certain places are better off without footer
            if navigation.actionResult.footer {
                Footer(
                    footerButton: navigation.actionResult.footerButton,
                    navigationRequest: { navigationRequest in
                        navigation.perform(navigation: navigationRequest)
                    }
                )
                .padding(.horizontal)
                .padding(.vertical, 8)
                .background(Asset.bg000.swiftUIColor)
            }
        }
        .gesture(
            DragGesture().onEnded { drag in
                if drag.translation.width < -20 {
                    navigation.perform(navigation: .init(action: .goBack))
                }
            }
        )
        .alert(Localizable.navigationError.text, isPresented: $data.parsingAlert, actions: {})
    }
}
