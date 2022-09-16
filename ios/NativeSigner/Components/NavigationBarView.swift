//
//  NavigationBarView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 22/08/2022.
//

import SwiftUI

enum NavigationLeftButton: Equatable {
    case empty
    case arrow
    case xmark
}

enum NavigationRightButton: Equatable {
    case empty
    case more
    case action(LocalizedStringKey)
}

struct NavigationBarViewModel: Equatable {
    let title: String?
    let subtitle: String?
    let leftButton: NavigationLeftButton
    let rightButton: NavigationRightButton

    init(
        title: String? = nil,
        subtitle: String? = nil,
        leftButton: NavigationLeftButton = .empty,
        rightButton: NavigationRightButton = .empty
    ) {
        self.title = title
        self.subtitle = subtitle
        self.leftButton = leftButton
        self.rightButton = rightButton
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
            switch viewModel.leftButton {
            case .empty:
                Spacer().frame(width: Heights.navigationButton)
            case .arrow:
                NavbarButton(
                    action: { navigation.perform(navigation: .init(action: .goBack)) },
                    icon: Asset.arrowBack.swiftUIImage
                )
            case .xmark:
                NavbarButton(
                    action: { navigation.perform(navigation: .init(action: .goBack)) },
                    icon: Asset.xmarkButton.swiftUIImage
                )
            }
            Spacer()
            VStack {
                if let title = viewModel.title {
                    Text(title)
                        .font(Fontstyle.titleS.base)
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor).lineLimit(1)
                }
                if let subtitle = viewModel.subtitle {
                    Text(subtitle)
                        .font(Fontstyle.captionM.base)
                        .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                }
            }

            Spacer()
            switch viewModel.rightButton {
            case .empty:
                Spacer().frame(width: Heights.navigationButton)
            case .more:
                NavbarButton(
                    action: actionModel.rightBarMenuAction,
                    icon: Asset.moreDots.swiftUIImage
                )
            case let .action(title):
                NavbarActionButton(action: actionModel.rightBarMenuAction, title: title)
            }
        }
        .padding([.leading, .trailing], Spacing.extraExtraSmall)
        .frame(maxWidth: .infinity)
        .frame(height: Heights.navigationBarHeight)
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
                    leftButton: .empty,
                    rightButton: .empty
                )
            )
            NavigationBarView(
                navigation: NavigationCoordinator(),
                viewModel: NavigationBarViewModel(
                    title: "Key Sets",
                    leftButton: .arrow,
                    rightButton: .more
                )
            )
            NavigationBarView(
                navigation: NavigationCoordinator(),
                viewModel: NavigationBarViewModel(
                    title: "Key Sets",
                    leftButton: .arrow,
                    rightButton: .empty
                )
            )
            NavigationBarView(
                navigation: NavigationCoordinator(),
                viewModel: NavigationBarViewModel(
                    title: "Key Sets",
                    leftButton: .empty,
                    rightButton: .more
                )
            )
            NavigationBarView(
                navigation: NavigationCoordinator(),
                viewModel: NavigationBarViewModel(
                    title: "Public Key",
                    subtitle: "Derived Key",
                    leftButton: .xmark,
                    rightButton: .more
                )
            )
            NavigationBarView(
                navigation: NavigationCoordinator(),
                viewModel: NavigationBarViewModel(
                    title: "Create Derived Key",
                    leftButton: .xmark,
                    rightButton: .action(Localizable.done.key)
                )
            )
        }
        .preferredColorScheme(.dark)
        .previewLayout(.sizeThatFits)
    }
}
