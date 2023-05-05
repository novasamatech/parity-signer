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
    @StateObject var viewModel: ViewModel

    var body: some View {
        HStack {
            TabBarButton(
                viewModel: viewModel.keysTab,
                onTap: viewModel.onKeysTap
            )
            CentralTabBarButton(
                viewModel: viewModel.scannerTab,
                onQRCodeTap: viewModel.onQRCodeTap
            )
            TabBarButton(
                viewModel: viewModel.settingsTab,
                onTap: viewModel.onSettingsTap
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

extension TabBarView {
    final class ViewModel: ObservableObject {
        let onQRCodeTap: () -> Void
        let onKeysTap: () -> Void
        let onSettingsTap: () -> Void
        let selectedTab: Tab
        @Published var keysTab: TabViewModel!
        @Published var scannerTab: TabViewModel = TabViewModelBuilder().build(for: .scanner, isSelected: false)
        @Published var settingsTab: TabViewModel!

        init(
            selectedTab: Tab,
            onQRCodeTap: @escaping () -> Void,
            onKeysTap: @escaping () -> Void,
            onSettingsTap: @escaping () -> Void
        ) {
            self.selectedTab = selectedTab
            self.onQRCodeTap = onQRCodeTap
            self.onKeysTap = onKeysTap
            self.onSettingsTap = onSettingsTap
            keysTab = TabViewModelBuilder().build(for: .keys, isSelected: selectedTab == .keys)
            settingsTab = TabViewModelBuilder().build(for: .settings, isSelected: selectedTab == .settings)
        }
    }
}

/// View mimicing single `.tabItem()` within `TabView` equivalent view (here: TabBarView)
private struct TabBarButton: View {
    private let viewModel: TabViewModel
    let onTap: () -> Void

    init(
        viewModel: TabViewModel,
        onTap: @escaping () -> Void
    ) {
        self.viewModel = viewModel
        self.onTap = onTap
    }

    var body: some View {
        Button(
            action: onTap,
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
        TabBarView(viewModel: .mock)
            .previewLayout(.sizeThatFits)
            .environmentObject(NavigationCoordinator())
    }
}

extension TabBarView.ViewModel {
    static let mock = TabBarView.ViewModel(
        selectedTab: .keys,
        onQRCodeTap: {},
        onKeysTap: {},
        onSettingsTap: {}
    )
}
