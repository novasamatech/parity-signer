//
//  String+Utilities.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 11/09/2022.
//

import Foundation

extension String {
    private enum Constants {
        static let defaultTruncate = 6
    }

    /// Truncates middle of `String` value
    /// - Parameter length: how many characters to leave on leading and trailing side
    /// - Returns: truncated `Self`
    func truncateMiddle(length: Int = Constants.defaultTruncate) -> String {
        count > length * 2 ? prefix(length) + "..." + suffix(length) : self
    }
}
