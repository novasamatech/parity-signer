//
//  NavigationBarView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 22/08/2022.
//

import SwiftUI

struct NavigationBarViewModel: Equatable {
    let title: String?
    let isBackButtonVisible: Bool
    let isRightBarMenuButtonVisible: Bool

    init(
        title: String? = nil,
        isBackButtonVisible: Bool = false,
        isRightBarMenuButtonVisible: Bool = false
    ) {
        self.title = title
        self.isBackButtonVisible = isBackButtonVisible
        self.isRightBarMenuButtonVisible = isRightBarMenuButtonVisible
    }
}

struct NavigationBarActionModel {
    let rightBarMenuAction: () -> Void
}

/// UI component that mimics system `NavigationView` and should be used as `NavigationBar` equivalent in `UIKit`
///
/// As we can't switch to `NavigationView` just yet, this should us in the meantime
struct NavigationBarView: View {
    @ObservedObject private var navigation: NavigationCoordinator
    private let viewModel: NavigationBarViewModel
    private let actionModel: NavigationBarActionModel

    init(
        navigation: NavigationCoordinator,
        viewModel: NavigationBarViewModel,
        actionModel: NavigationBarActionModel? = nil
    ) {
        self.navigation = navigation
        self.viewModel = viewModel
        self.actionModel = actionModel ?? .init(rightBarMenuAction: {
            navigation.perform(navigation: .init(action: .rightButtonAction))
        })
    }

    var body: some View {
        HStack(alignment: .center) {
            if viewModel.isBackButtonVisible {
                NavbarButton(
                    action: { navigation.perform(navigation: .init(action: .goBack)) },
                    icon: Asset.arrowBack.swiftUIImage
                )
            } else {
                Spacer().frame(width: Heights.navigationButton)
            }
            Spacer().frame(maxWidth: .infinity)
            if let title = viewModel.title {
                Text(title)
                    .font(Fontstyle.titleS.base)
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
            }
            Spacer().frame(maxWidth: .infinity)
            if viewModel.isRightBarMenuButtonVisible {
                NavbarButton(
                    action: actionModel.rightBarMenuAction,
                    icon: Asset.moreDots.swiftUIImage
                )
            } else {
                Spacer().frame(width: Heights.navigationButton)
            }
        }
        .frame(maxWidth: .infinity)
        .frame(height: 64)
        .background(Asset.backgroundSystem.swiftUIColor)
    }
}

struct NavigationBarView_Previews: PreviewProvider {
    static var previews: some View {
        VStack {
            NavigationBarView(
                navigation: NavigationCoordinator(),
                viewModel: NavigationBarViewModel(
                    title: "Key Sets",
                    isBackButtonVisible: false,
                    isRightBarMenuButtonVisible: false
                )
            )
            NavigationBarView(
                navigation: NavigationCoordinator(),
                viewModel: NavigationBarViewModel(
                    title: "Key Sets",
                    isBackButtonVisible: true,
                    isRightBarMenuButtonVisible: true
                )
            )
            NavigationBarView(
                navigation: NavigationCoordinator(),
                viewModel: NavigationBarViewModel(
                    title: "Key Sets",
                    isBackButtonVisible: true,
                    isRightBarMenuButtonVisible: false
                )
            )
            NavigationBarView(
                navigation: NavigationCoordinator(),
                viewModel: NavigationBarViewModel(
                    title: "Key Sets",
                    isBackButtonVisible: false,
                    isRightBarMenuButtonVisible: true
                )
            )
        }
        .preferredColorScheme(.dark)
        .previewLayout(.sizeThatFits)
        VStack {
            NavigationBarView(
                navigation: NavigationCoordinator(),
                viewModel: NavigationBarViewModel(
                    title: "Key Sets",
                    isBackButtonVisible: false,
                    isRightBarMenuButtonVisible: false
                )
            )
            NavigationBarView(
                navigation: NavigationCoordinator(),
                viewModel: NavigationBarViewModel(
                    title: "Key Sets",
                    isBackButtonVisible: true,
                    isRightBarMenuButtonVisible: true
                )
            )
            NavigationBarView(
                navigation: NavigationCoordinator(),
                viewModel: NavigationBarViewModel(
                    title: "Key Sets",
                    isBackButtonVisible: true,
                    isRightBarMenuButtonVisible: false
                )
            )
            NavigationBarView(
                navigation: NavigationCoordinator(),
                viewModel: NavigationBarViewModel(
                    title: "Key Sets",
                    isBackButtonVisible: false,
                    isRightBarMenuButtonVisible: true
                )
            )
        }
        .preferredColorScheme(.light)
        .previewLayout(.sizeThatFits)
    }
}
