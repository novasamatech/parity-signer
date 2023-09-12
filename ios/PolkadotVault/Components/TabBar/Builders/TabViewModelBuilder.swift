//
//  TabViewModelBuilder.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 18/08/2022.
//

import SwiftUI

/// View model to render `TabBarButton`
struct TabViewModel: Equatable {
    let action: Action?
    let isActive: Bool
    let icon: Image
    let label: Text
    let tab: Tab
}

/// Builds view model for single `TabBarButton` based on current tab bar state
final class TabViewModelBuilder {
    func build(for tab: Tab, isSelected: Bool) -> TabViewModel {
        TabViewModel(
            action: tab.action,
            isActive: isSelected,
            icon: icon(for: tab, isSelected: isSelected),
            label: label(for: tab),
            tab: tab
        )
    }
}

private extension TabViewModelBuilder {
    func icon(for tab: Tab, isSelected _: Bool) -> Image {
        switch tab {
        case .keys:
            return Asset.tabbarKeys.swiftUIImage
        case .scanner:
            return Asset.tabbarScanner.swiftUIImage
        }
    }

    func label(for tab: Tab) -> Text {
        switch tab {
        case .keys:
            return Localizable.TabBar.keys.text
        case .scanner:
            return Localizable.TabBar.scanner.text
        }
    }
}
