//
//  FileManagingProtocol.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 03/08/2022.
//

import Foundation

// sourcery: AutoMockable
/// Protocol reflecting `FileManager` functionality
protocol FileManagingProtocol: AnyObject {
    /// Returns a Boolean value that indicates whether a file or directory exists at a specified path.
    /// - Parameter path: The path of the file or directory. If path begins with a tilde (~), it must first be expanded
    /// with expandingTildeInPath; otherwise, this method returns false
    /// - Returns: `true` if a file at the specified path exists, or `false` if the file does not exist or its existence
    /// could not be determined.
    func fileExists(atPath path: String) -> Bool
    /// Removes the file or directory at the specified URL.
    /// - Parameter URL: A file URL specifying the file or directory to remove. If the URL specifies a directory, the
    /// contents of that directory are recursively removed.
    func removeItem(at URL: URL) throws
    /// Removes the file or directory at the specified path.
    func removeItem(atPath path: String) throws

    /// Copies the file at the specified URL to a new location synchronously.
    /// - Parameters:
    ///   - srcURL: The file URL that identifies the file you want to copy. The URL in this parameter must not be a file
    /// reference URL. This parameter must not be nil.
    ///   - dstURL: The URL at which to place the copy of srcURL. The URL in this parameter must not be a file reference
    /// URL and must include the name of the file in its new location. This parameter must not be nil.
    func copyItem(at srcURL: URL, to dstURL: URL) throws

    /// Locates and optionally creates the specified common directory in a domain.
    /// - Parameters:
    ///   - directory: The search path directory. The supported values are described in FileManager.SearchPathDirectory.
    ///   - domain: The file system domain to search. The value for this parameter is one of the constants described in
    /// FileManager.SearchPathDomainMask.
    ///   - url: The file URL used to determine the location of the returned URL. Only the volume of this parameter is
    /// used.
    ///   - shouldCreate: Whether to create the directory if it does not already exist.
    /// - Returns: The NSURL for the requested directory.
    func url(
        for directory: FileManager.SearchPathDirectory,
        in domain: FileManager.SearchPathDomainMask,
        appropriateFor url: URL?,
        create shouldCreate: Bool
    ) throws -> URL
}

extension FileManager: FileManagingProtocol {}
