//
//  TabBarView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 17/08/2022.
//

import SwiftUI

/// UI component that mimics system `TabView` and handles bottom navigation
///
/// `body` should rely on system `TabView` or its subclass, when navigation is moved to native system
struct TabBarView: View {
    @Environment(\.colorScheme) var deviceColorScheme: ColorScheme

    /// Handles navigation when `Tab` is selected
    @EnvironmentObject private var navigation: NavigationCoordinator
    /// View model reflecting selected tab in bottom navigation
    ///
    /// For now this value is based on `FooterButton` from `ActionResult`, but when navigation is moved
    /// to native system this should be `private let` and not derived from external view models
    @Binding var selectedTab: Tab

    private let viewModelBuilder = TabViewModelBuilder()

    var body: some View {
        HStack {
            ForEach(Tab.allCases, id: \.self) { tab in
                if tab == .scanner {
                    CentralTabBarButton(viewModel: viewModelBuilder.build(for: tab, isSelected: tab == selectedTab))
                } else {
                    TabBarButton(
                        viewModel: viewModelBuilder.build(for: tab, isSelected: tab == selectedTab)
                    )
                }
            }
        }
        .frame(height: Heights.tabbarHeight)
        .background(Asset.backgroundSecondary.swiftUIColor)
        .overlay(
            Divider().background(deviceColorScheme == .dark ? Asset.fill30LightOnly.swiftUIColor : Color.clear),
            alignment: .top
        )
    }
}

/// View mimicing single `.tabItem()` within `TabView` equivalent view (here: TabBarView)
private struct TabBarButton: View {
    @EnvironmentObject private var navigation: NavigationCoordinator

    private let viewModel: TabViewModel

    init(
        viewModel: TabViewModel
    ) {
        self.viewModel = viewModel
    }

    var body: some View {
        Button(
            action: {
                if let action = viewModel.action {
                    navigation.perform(navigation: .init(action: action))
                } else {
                    navigation.shouldPresentQRScanner.toggle()
                }
            },
            label: {
                VStack {
                    viewModel.icon
                        .frame(height: Heights.tabbarAssetHeight, alignment: .center)
                        .padding(.bottom, -Spacing.extraExtraSmall)
                    viewModel.label
                        .font(PrimaryFont.captionS.font)
                }
                .foregroundColor(
                    viewModel.isActive ?
                        Asset.textAndIconsPrimary.swiftUIColor :
                        Asset.textAndIconsTertiary.swiftUIColor
                )
            }
        )
        .frame(maxWidth: .infinity)
    }
}

/// View mimicing single `.tabItem()` within `TabView` equivalent view (here: TabBarView)
private struct CentralTabBarButton: View {
    @EnvironmentObject private var navigation: NavigationCoordinator

    private let viewModel: TabViewModel

    init(
        viewModel: TabViewModel
    ) {
        self.viewModel = viewModel
    }

    var body: some View {
        Button(
            action: {
                if let action = viewModel.action {
                    navigation.perform(navigation: .init(action: action))
                } else {
                    navigation.shouldPresentQRScanner.toggle()
                }
            },
            label: {
                viewModel.icon
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
            }
        )
        .frame(height: Heights.tabbarScannerHeight, alignment: .center)
        .frame(maxWidth: .infinity)
        .background(
            RoundedRectangle(cornerRadius: CornerRadius.extraLarge)
                .stroke(Asset.fill12.swiftUIColor, lineWidth: 2)
                .cornerRadius(CornerRadius.extraLarge)
        )
        .padding(.horizontal, Spacing.medium)
    }
}

/// To test preview with different `Tab` selected, just substitute `selectedTab` with
/// `Binding<Tab>.constant(<any enum Tab value here>)`
struct TabBarView_Previews: PreviewProvider {
    static var previews: some View {
        TabBarView(
            selectedTab: Binding<Tab>.constant(.keys)
        )
        .previewLayout(.sizeThatFits)
        .environmentObject(NavigationCoordinator())
    }
}
