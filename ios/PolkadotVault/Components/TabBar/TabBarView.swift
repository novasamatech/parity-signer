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
    let selectedTab: Tab
    let onQRCodeTap: () -> Void
    private let viewModelBuilder = TabViewModelBuilder()

    init(
        selectedTab: Tab,
        onQRCodeTap: @escaping () -> Void
    ) {
        self.selectedTab = selectedTab
        self.onQRCodeTap = onQRCodeTap
    }

    var body: some View {
        HStack {
            TabBarButton(
                viewModel: viewModelBuilder.build(for: .keys, isSelected: selectedTab == .keys)
            )
            CentralTabBarButton(
                viewModel: viewModelBuilder.build(for: .scanner, isSelected: false),
                onQRCodeTap: onQRCodeTap
            )
            TabBarButton(
                viewModel: viewModelBuilder.build(for: .settings, isSelected: selectedTab == .settings)
            )
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
    let onQRCodeTap: () -> Void

    init(
        viewModel: TabViewModel,
        onQRCodeTap: @escaping () -> Void
    ) {
        self.viewModel = viewModel
        self.onQRCodeTap = onQRCodeTap
    }

    var body: some View {
        Button(
            action: onQRCodeTap,
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
            selectedTab: .keys,
            onQRCodeTap: {}
        )
        .previewLayout(.sizeThatFits)
        .environmentObject(NavigationCoordinator())
    }
}
