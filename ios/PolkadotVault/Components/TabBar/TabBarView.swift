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
        }
        .frame(height: Heights.tabbarHeight)
        .background(Asset.backgroundSecondary.swiftUIColor)
        .overlay(
            Divider().background(deviceColorScheme == .dark ? Asset.fill30LightOnly.swiftUIColor : Color.clear),
            alignment: .top
        )
        .onChange(of: viewModel.selectedTab, perform: viewModel.onTabChange(_:))
    }
}

extension TabBarView {
    final class ViewModel: ObservableObject {
        let onQRCodeTap: () -> Void
        let onKeysTap: () -> Void
        @Binding var selectedTab: Tab
        @Published var keysTab: TabViewModel!
        @Published var scannerTab: TabViewModel = TabViewModelBuilder().build(for: .scanner, isSelected: false)

        init(
            selectedTab: Binding<Tab>,
            onQRCodeTap: @escaping () -> Void,
            onKeysTap: @escaping () -> Void
        ) {
            _selectedTab = selectedTab
            self.onQRCodeTap = onQRCodeTap
            self.onKeysTap = onKeysTap
            onTabChange(selectedTab.wrappedValue)
        }

        func onTabChange(_ tab: Tab) {
            keysTab = TabViewModelBuilder().build(for: .keys, isSelected: tab == .keys)
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

#if DEBUG
    struct TabBarView_Previews: PreviewProvider {
        static var previews: some View {
            TabBarView(viewModel: .mock)
                .previewLayout(.sizeThatFits)
        }
    }
#endif

extension TabBarView.ViewModel {
    static let mock = TabBarView.ViewModel(
        selectedTab: .constant(.keys),
        onQRCodeTap: {},
        onKeysTap: {}
    )
}
