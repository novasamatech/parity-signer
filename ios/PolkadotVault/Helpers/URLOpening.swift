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
}

extension UIApplication: URLOpening {}
