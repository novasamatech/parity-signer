//
//  KeyDetailsActionModel.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 31/08/2022.
//

import Foundation

/// Model of available actions on `Key Details` screen
struct KeyDetailsActionModel {
    /// Navigation action for selecting main `Address Key`
    let addressKeyNavigation: Navigation
    /// Collection of navigation actions for tapping on `Derived Key`
    let derivedKeysNavigation: [Navigation]
    /// Navigation for `Create Derived Key`
    let createDerivedKey: Navigation = .init(action: .newKey)
    /// Optional alert closure when tapping on `Create Derived Key`
    let alertClosure: (() -> Void)?
    /// Name of seed to be removed with `Remove Seed` action
    let removeSeed: String
}

extension KeyDetailsActionModel {
    init(_ keys: MKeys, alert: Bool, alertShow: @escaping () -> Void) {
        addressKeyNavigation = .init(action: .selectKey, details: keys.root.addressKey)
        derivedKeysNavigation = keys.set
            .sorted(by: { $0.path < $1.path })
            .map { .init(action: .selectKey, details: $0.addressKey) }
        alertClosure = alert ? alertShow : nil
        removeSeed = keys.root.seedName
    }
}
