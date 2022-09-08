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
            if navigation.shouldSkipInjectedViews {
                ZStack {
                    VStack(spacing: 0) {
                        ScreenSelectorView(data: data, navigation: navigation)
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
            } else {
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
            }
            if navigation.actionResult.footer {
                TabBarView(
                    navigation: navigation,
                    selectedTab: $navigation.selectedTab
                )
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
