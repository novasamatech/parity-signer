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
    case settings
    case action(LocalizedStringKey)
    case activeAction(LocalizedStringKey, Binding<Bool>)
    case questionmark
}

enum NavigationBarTitle {
    case empty
    case title(String)
    case subtitle(title: String, subtitle: String)
    case progress(current: Int, upTo: Int)
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
    let title: NavigationBarTitle
    let leftButtons: [NavigationButtonModel]
    let rightButtons: [NavigationButtonModel]
    let backgroundColor: Color

    init(
        title: NavigationBarTitle = .empty,
        leftButtons: [NavigationButtonModel] = [],
        rightButtons: [NavigationButtonModel] = [],
        backgroundColor: Color = .backgroundPrimary
    ) {
        self.title = title
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
                .padding(.horizontal, Spacing.extraExtraSmall)
                titleView(viewModel.title)
            }
            .frame(maxWidth: .infinity)
            .frame(height: Heights.navigationBarHeight)
            .background(viewModel.backgroundColor)
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
                icon: Image(.arrowBack)
            )
        case .xmark:
            NavbarButton(
                action: button.action,
                icon: Image(.xmarkButton)
            )
        case .more:
            NavbarButton(
                action: button.action,
                icon: Image(.moreDots)
            )
        case .settings:
            NavbarButton(
                action: button.action,
                icon: Image(.tabbarSettings)
            )
        case .plus:
            NavbarButton(
                action: button.action,
                icon: Image(ImageResource.plus)
            )
        case .questionmark:
            NavbarButton(
                action: button.action,
                icon: Image(.navbarQuestion)
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

    @ViewBuilder
    func titleView(_ title: NavigationBarTitle) -> some View {
        HStack(alignment: .center, spacing: 0) {
            Spacer()
            switch title {
            case .empty:
                EmptyView()
            case let .title(title):
                VStack {
                    Text(title)
                        .font(PrimaryFont.titleS.font)
                        .foregroundColor(.textAndIconsPrimary).lineLimit(1)
                }
            case let .subtitle(title, subtitle):
                VStack {
                    Text(title)
                        .font(PrimaryFont.titleS.font)
                        .foregroundColor(.textAndIconsPrimary).lineLimit(1)
                    Text(subtitle)
                        .font(PrimaryFont.captionM.font)
                        .foregroundColor(.textAndIconsSecondary)
                }
            case let .progress(current, upTo):
                progressView(current, upTo: upTo)
            }
            Spacer()
        }
    }

    @ViewBuilder
    func progressView(_ current: Int, upTo: Int) -> some View {
        ForEach(Array(0 ..< upTo).indices, id: \.self) { index in
            progressViewElement(isActive: index < current)
                .padding(.leading, index != 0 ? Spacing.extraExtraSmall : 0)
        }
    }

    @ViewBuilder
    func progressViewElement(isActive: Bool) -> some View {
        RoundedRectangle(cornerRadius: CornerRadius.extraLarge)
            .frame(width: Heights.navigationBarProgressViewWidth, height: Heights.navigationBarProgressViewHeight)
            .foregroundColor(
                isActive ? .pink500 : .fill12
            )
    }
}

#if DEBUG
    struct NavigationBarView_Previews: PreviewProvider {
        static var previews: some View {
            VStack {
                NavigationBarView(
                    viewModel: NavigationBarViewModel(
                        title: .title("Key Sets"),
                        leftButtons: [.init(type: .empty)],
                        rightButtons: [.init(type: .empty)]
                    )
                )
                NavigationBarView(
                    viewModel: NavigationBarViewModel(
                        title: .title("Key Sets"),
                        leftButtons: [.init(type: .arrow)],
                        rightButtons: [.init(type: .more)]
                    )
                )
                NavigationBarView(
                    viewModel: NavigationBarViewModel(
                        title: .title("Key Sets"),
                        leftButtons: [.init(type: .arrow)],
                        rightButtons: [.init(type: .empty)]
                    )
                )
                NavigationBarView(
                    viewModel: NavigationBarViewModel(
                        title: .title("Key Sets"),
                        leftButtons: [.init(type: .empty)],
                        rightButtons: [.init(type: .more)]
                    )
                )
                NavigationBarView(
                    viewModel: NavigationBarViewModel(
                        title: .subtitle(title: "Key Sets", subtitle: "Derived Key"),
                        leftButtons: [.init(type: .xmark)],
                        rightButtons: [.init(type: .plus), .init(type: .more)]
                    )
                )
                NavigationBarView(
                    viewModel: NavigationBarViewModel(
                        title: .title("Create Derived Key"),
                        leftButtons: [.init(type: .xmark)],
                        rightButtons: [.init(type: .action("Done"))]
                    )
                )
            }
            .preferredColorScheme(.dark)
            .previewLayout(.sizeThatFits)
        }
    }
#endif
