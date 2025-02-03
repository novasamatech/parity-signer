//
//  URLOpening.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 16/01/2024.
//

import Foundation
import UIKit

// sourcery: AutoMockable
protocol URLOpening: AnyObject {
    func canOpenURL(_ url: URL) -> Bool
    func open(_ url: URL)
    func open(
        _ url: URL,
        options: [UIApplication.OpenExternalURLOptionsKey: Any],
        completionHandler completion: (@MainActor @Sendable (Bool) -> Void)?
    )
}

extension UIApplication: URLOpening {
    func open(_ url: URL) {
        open(url, options: [:], completionHandler: nil)
    }
}
