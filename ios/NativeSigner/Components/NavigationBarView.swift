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
    case questionmark
}

struct NavigationBarViewModel: Equatable {
    let title: String?
    let subtitle: String?
    let leftButton: NavigationLeftButton
    let rightButton: NavigationRightButton
    let backgroundColor: Color

    init(
        title: String? = nil,
        subtitle: String? = nil,
        leftButton: NavigationLeftButton = .empty,
        rightButton: NavigationRightButton = .empty,
        backgroundColor: Color = Asset.backgroundPrimary.swiftUIColor
    ) {
        self.title = title
        self.subtitle = subtitle
        self.leftButton = leftButton
        self.rightButton = rightButton
        self.backgroundColor = backgroundColor
    }
}

struct NavigationBarActionModel {
    let leftBarMenuAction: (() -> Void)?
    let rightBarMenuAction: (() -> Void)?

    init(
        leftBarMenuAction: (() -> Void)? = nil,
        rightBarMenuAction: (() -> Void)? = nil
    ) {
        self.leftBarMenuAction = leftBarMenuAction
        self.rightBarMenuAction = rightBarMenuAction
    }
}

/// UI component that mimics system `NavigationView` and should be used as `NavigationBar` equivalent in `UIKit`
///
/// As we can't switch to `NavigationView` just yet, this should us in the meantime
struct NavigationBarView: View {
    @EnvironmentObject private var navigation: NavigationCoordinator
    private let viewModel: NavigationBarViewModel
    private let actionModel: NavigationBarActionModel

    init(
        viewModel: NavigationBarViewModel,
        actionModel: NavigationBarActionModel = .init()
    ) {
        self.viewModel = viewModel
        self.actionModel = actionModel
    }

    var body: some View {
        HStack(alignment: .center) {
            switch viewModel.leftButton {
            case .empty:
                Spacer().frame(width: Heights.navigationButton)
            case .arrow:
                NavbarButton(
                    action: leftButtonAction(),
                    icon: Asset.arrowBack.swiftUIImage
                )
            case .xmark:
                NavbarButton(
                    action: leftButtonAction(),
                    icon: Asset.xmarkButton.swiftUIImage
                )
            }
            Spacer()
            VStack {
                if let title = viewModel.title {
                    Text(title)
                        .font(PrimaryFont.titleS.font)
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor).lineLimit(1)
                }
                if let subtitle = viewModel.subtitle {
                    Text(subtitle)
                        .font(PrimaryFont.captionM.font)
                        .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                }
            }

            Spacer()
            switch viewModel.rightButton {
            case .empty:
                Spacer().frame(width: Heights.navigationButton)
            case .more:
                NavbarButton(
                    action: actionModel.rightBarMenuAction ?? {},
                    icon: Asset.moreDots.swiftUIImage
                )
            case .questionmark:
                NavbarButton(
                    action: actionModel.rightBarMenuAction ?? {},
                    icon: Asset.navbarQuestion.swiftUIImage
                )
            case let .action(title):
                NavbarActionButton(
                    action: actionModel.rightBarMenuAction ?? {},
                    title: title
                )
            }
        }
        .padding([.leading, .trailing], Spacing.extraExtraSmall)
        .frame(maxWidth: .infinity)
        .frame(height: Heights.navigationBarHeight)
    }

    private func leftButtonAction() -> () -> Void {
        actionModel.leftBarMenuAction ??
            { navigation.perform(navigation: .init(action: .goBack)) }
    }
}

struct NavigationBarView_Previews: PreviewProvider {
    static var previews: some View {
        VStack {
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: "Key Sets",
                    leftButton: .empty,
                    rightButton: .empty
                ),
                actionModel: .init(rightBarMenuAction: {})
            )
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: "Key Sets",
                    leftButton: .arrow,
                    rightButton: .more
                ),
                actionModel: .init(rightBarMenuAction: {})
            )
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: "Key Sets",
                    leftButton: .arrow,
                    rightButton: .empty
                ),
                actionModel: .init(rightBarMenuAction: {})
            )
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: "Key Sets",
                    leftButton: .empty,
                    rightButton: .more
                ),
                actionModel: .init(rightBarMenuAction: {})
            )
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: "Public Key",
                    subtitle: "Derived Key",
                    leftButton: .xmark,
                    rightButton: .more
                ),
                actionModel: .init(rightBarMenuAction: {})
            )
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: "Create Derived Key",
                    leftButton: .xmark,
                    rightButton: .action(Localizable.done.key)
                ),
                actionModel: .init(rightBarMenuAction: {})
            )
        }
        .preferredColorScheme(.dark)
        .previewLayout(.sizeThatFits)
        .environmentObject(NavigationCoordinator())
    }
}
