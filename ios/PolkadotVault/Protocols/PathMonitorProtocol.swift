//
//  PathMonitorProtocol.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 02/08/2022.
//

import Foundation
import Network

/// Protocol that reflects `PathMonitor` functionality
protocol PathMonitorProtocol: AnyObject {
    /// Set a block to be called when the connection's path has changed, which may be called
    /// multiple times until the connection is cancelled.
    @preconcurrency
    var pathUpdateHandler: (@Sendable (NWPath) -> Void)? { get set }
    /// Start the connection and provide a dispatch queue for callback blocks.
    ///
    /// Starts the connection, which will cause the connection to evaluate its path, do resolution and try to become
    /// ready (connected). NWConnection establishment is asynchronous. A stateUpdateHandler may be used to determine
    /// when the state changes. If the connection cannot be established, the state will transition to waiting with
    /// an associated error describing the reason. If an unrecoverable error is encountered, the state will
    /// transition to failed with an associated NWError value. If the connection is established, the state will
    /// transition to ready.
    ///
    /// Start should only be called once on a connection, and multiple calls to start will
    /// be ignored.
    /// - Parameter queue:
    func start(queue: DispatchQueue)
}

extension NWPathMonitor: PathMonitorProtocol {}
