//
//  NavigationBarView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 22/08/2022.
//

import SwiftUI

enum NavigationButton {
    case empty
    case arrow
    case xmark
    case more
    case plus
    case action(LocalizedStringKey)
    case activeAction(LocalizedStringKey, Binding<Bool>)
    case questionmark
}

struct NavigationButtonModel: Identifiable {
    let id = UUID()
    let type: NavigationButton
    let action: () -> Void

    init(
        type: NavigationButton = .empty,
        action: @escaping () -> Void = {}
    ) {
        self.type = type
        self.action = action
    }
}

struct NavigationBarViewModel {
    let title: String?
    let subtitle: String?
    let leftButtons: [NavigationButtonModel]
    let rightButtons: [NavigationButtonModel]
    let backgroundColor: Color

    init(
        title: String? = nil,
        subtitle: String? = nil,
        leftButtons: [NavigationButtonModel] = [.init()],
        rightButtons: [NavigationButtonModel] = [.init()],
        backgroundColor: Color = Asset.backgroundPrimary.swiftUIColor
    ) {
        self.title = title
        self.subtitle = subtitle
        self.leftButtons = leftButtons
        self.rightButtons = rightButtons
        self.backgroundColor = backgroundColor
    }
}

/// UI component that mimics system `NavigationView` and should be used as `NavigationBar` equivalent in `UIKit`
///
/// As we can't switch to `NavigationView` just yet, this should us in the meantime
struct NavigationBarView: View {
    private let viewModel: NavigationBarViewModel

    init(
        viewModel: NavigationBarViewModel
    ) {
        self.viewModel = viewModel
    }

    var body: some View {
        HStack(alignment: .center) {
            ZStack {
                HStack(alignment: .center, spacing: Spacing.extraSmall) {
                    ForEach(viewModel.leftButtons, id: \.id) {
                        buttonView($0)
                    }
                    Spacer()
                    ForEach(viewModel.rightButtons, id: \.id) {
                        buttonView($0)
                    }
                }
                .padding([.leading, .trailing], Spacing.extraExtraSmall)
                HStack(alignment: .center, spacing: 0) {
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
                }
            }
            .frame(maxWidth: .infinity)
            .frame(height: Heights.navigationBarHeight)
        }
    }

    @ViewBuilder
    func buttonView(_ button: NavigationButtonModel) -> some View {
        switch button.type {
        case .empty:
            Spacer().frame(width: Heights.navigationButton)
        case .arrow:
            NavbarButton(
                action: button.action,
                icon: Asset.arrowBack.swiftUIImage
            )
        case .xmark:
            NavbarButton(
                action: button.action,
                icon: Asset.xmarkButton.swiftUIImage
            )
        case .more:
            NavbarButton(
                action: button.action,
                icon: Asset.moreDots.swiftUIImage
            )
        case .plus:
            NavbarButton(
                action: button.action,
                icon: Asset.plus.swiftUIImage
            )
        case .questionmark:
            NavbarButton(
                action: button.action,
                icon: Asset.navbarQuestion.swiftUIImage
            )
        case let .action(title):
            NavbarActionButton(
                action: button.action,
                title: title
            )
        case let .activeAction(title, isDisabled):
            NavbarActionButton(
                action: button.action,
                title: title,
                isDisabled: isDisabled
            )
        }
    }
}

#if DEBUG
    struct NavigationBarView_Previews: PreviewProvider {
        static var previews: some View {
            VStack {
                NavigationBarView(
                    viewModel: NavigationBarViewModel(
                        title: "Key Sets",
                        leftButtons: [.init(type: .empty)],
                        rightButtons: [.init(type: .empty)]
                    )
                )
                NavigationBarView(
                    viewModel: NavigationBarViewModel(
                        title: "Key Sets",
                        leftButtons: [.init(type: .arrow)],
                        rightButtons: [.init(type: .more)]
                    )
                )
                NavigationBarView(
                    viewModel: NavigationBarViewModel(
                        title: "Key Sets",
                        leftButtons: [.init(type: .arrow)],
                        rightButtons: [.init(type: .empty)]
                    )
                )
                NavigationBarView(
                    viewModel: NavigationBarViewModel(
                        title: "Key Sets",
                        leftButtons: [.init(type: .empty)],
                        rightButtons: [.init(type: .more)]
                    )
                )
                NavigationBarView(
                    viewModel: NavigationBarViewModel(
                        title: "Public Key",
                        subtitle: "Derived Key",
                        leftButtons: [.init(type: .xmark)],
                        rightButtons: [.init(type: .plus), .init(type: .more)]
                    )
                )
                NavigationBarView(
                    viewModel: NavigationBarViewModel(
                        title: "Create Derived Key",
                        leftButtons: [.init(type: .xmark)],
                        rightButtons: [.init(type: .action("Done"))]
                    )
                )
            }
            .preferredColorScheme(.dark)
            .previewLayout(.sizeThatFits)
            .environmentObject(NavigationCoordinator())
        }
    }
#endif
