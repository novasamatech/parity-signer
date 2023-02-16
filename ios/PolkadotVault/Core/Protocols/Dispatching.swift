//
//  Dispatching.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 05/08/2022.
//

import Foundation

/// Protocol that reflects `DispatchQueue` functionality
protocol Dispatching: AnyObject {
    /// Schedules a block asynchronously for execution and optionally associates it with a dispatch group.
    /// - parameter execute: The work item to be invoked on the queue.
    func async(execute work: @escaping @convention(block) () -> Void)
    /// Submits a work item to a dispatch queue for asynchronous execution after
    /// a specified time.
    /// - parameter deadline: the time after which the work item should be executed,
    /// given as a `DispatchTime`.
    func asyncAfter(deadline: DispatchTime, execute work: @escaping @convention(block) () -> Void)
    /// Submits a work item to a dispatch queue for asynchronous execution after
    /// a specified time.
    /// - parameter deadline: the time after which the work item should be executed,
    /// - parameter flags: flags to be enabled for queue
    /// given as a `DispatchTime`.
    func asyncAfter(deadline: DispatchTime, flags: DispatchWorkItemFlags, execute work: @escaping () -> Void)
    /// Submits a work item for execution using the specified attributes
    /// and returns the results from that item after it finishes executing.
    /// - parameter flags: Additional attributes to apply when executing the block.
    /// - parameter work: The work item containing the work to perform.
    /// - Returns: The return value of the item in the `work` parameter.
    func sync<T>(flags: DispatchWorkItemFlags, execute work: () throws -> T) rethrows -> T
    /// Submits a work item for execution using the specified attributes
    /// and returns the results from that item after it finishes executing.
    /// - parameter work: The work item containing the work to perform.
    /// - Returns: The return value of the item in the `work` parameter.
    func sync<T>(execute work: () throws -> T) rethrows -> T
}

extension DispatchQueue: Dispatching {
    func async(execute work: @escaping @convention(block) () -> Void) {
        async(group: nil, qos: DispatchQoS.default, flags: [], execute: work)
    }

    func asyncAfter(deadline: DispatchTime, execute work: @escaping @convention(block) () -> Void) {
        asyncAfter(deadline: deadline, qos: DispatchQoS.default, flags: [], execute: work)
    }

    func asyncAfter(deadline: DispatchTime, flags: DispatchWorkItemFlags, execute work: @escaping () -> Void) {
        asyncAfter(deadline: deadline, qos: DispatchQoS.default, flags: flags, execute: work)
    }
}
