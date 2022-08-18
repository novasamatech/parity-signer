//
//  TabBarView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 17/08/2022.
//

import SwiftUI

/// UI component that mimics system `TabView` and handles bottom navigation
///
/// `body` should rely on system `TabView` or its subclass, when navigation is moved to native system
struct TabBarView: View {
    /// Handles navigation when `Tab` is selected
    @ObservedObject var navigation: NavigationCoordinator
    /// View model reflecting selected tab in bottom navigation
    ///
    /// For now this value is based on `FooterButton` from `ActionResult`, but when navigation is moved
    /// to native system this should be `private let` and not derived from external view models
    @Binding var selectedTab: Tab

    private let viewModelBuilder = TabViewModelBuilder()

    var body: some View {
        HStack {
            ForEach(Tab.allCases, id: \.self) { tab in
                TabBarButton(
                    navigation: navigation,
                    viewModel: viewModelBuilder.build(for: tab, isSelected: tab == selectedTab)
                )
            }
        }
        .frame(height: 49)
        .background(Asset.backgroundPrimary.swiftUIColor)
    }
}

/// View mimicing single `.tabItem()` within `TabView` equivalent view (here: TabBarView)
private struct TabBarButton: View {
    @ObservedObject var navigation: NavigationCoordinator

    private let viewModel: TabViewModel

    init(
        navigation: NavigationCoordinator,
        viewModel: TabViewModel
    ) {
        self.navigation = navigation
        self.viewModel = viewModel
    }

    var body: some View {
        Button(
            action: {
                navigation.perform(navigation: .init(action: viewModel.action))
            },
            label: {
                VStack {
                    viewModel.icon
                        .frame(height: 28, alignment: .center)
                    viewModel.label
                        .font(Fontstyle.captionS.base)
                }
                .foregroundColor(
                    viewModel.isActive ?
                        Asset.accentPink500.swiftUIColor :
                        Asset.textAndIconsTertiary.swiftUIColor
                )
            }
        )
        .frame(maxWidth: .infinity)
    }
}

/// To test preview with different `Tab` selected, just substitute `selectedTab` with `Binding<Tab>.constant(<any enum Tab value here>)`
// struct TabBarView_Previews: PreviewProvider {
//    static var previews: some View {
//        TabBarView(
//            navigation: NavigationCoordinator(),
//            selectedTab: Binding<Tab>.constant(.logs)
//        )
//        .previewLayout(.sizeThatFits)
//    }
// }
