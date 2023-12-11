//
//  BundleProtocol.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 03/08/2022.
//

import Foundation

// sourcery: AutoMockable
/// Protocol reflecting `Bundle` functionality
protocol BundleProtocol: AnyObject {
    /// Returns the file URL for the resource identified by the specified name and file extension.
    /// - Parameters:
    ///   - name: The name of the resource file. If you specify nil, the method returns the first resource file it finds
    /// with the specified extension.
    ///   - ext: The extension of the resource file. If extension is an empty string or nil, the extension is assumed
    /// not
    /// to exist and the file URL is the first file encountered that exactly matches name.
    /// - Returns: The file URL for the resource file or nil if the file could not be located.
    func url(forResource name: String?, withExtension ext: String?) -> URL?
}

extension Bundle: BundleProtocol {}
