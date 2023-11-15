//
//  AirgapMediator.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 15/02/2023.
//

import Combine
import Foundation
import Network

enum AirgapComponent: Equatable, Hashable {
    case aiplaneMode
    case wifi
}

protocol AirgapMediating: AnyObject {
    func startMonitoringAirgap(_ update: @escaping (Bool, Bool) -> Void)
}

final class AirgapMediator: AirgapMediating {
    private let adaptee: NWPathMonitor
    private let monitoringQueue: DispatchQueue
    private let notificationQueue: DispatchQueue

    var isWifiOn: AnyPublisher<Bool, Never>!
    var isAirplaneModeOn: AnyPublisher<Bool, Never>!

    init(
        adaptee: NWPathMonitor = NWPathMonitor(),
        monitoringQueue: DispatchQueue = DispatchQueue.global(qos: .background),
        notificationQueue: DispatchQueue = DispatchQueue.main
    ) {
        self.adaptee = adaptee
        self.monitoringQueue = monitoringQueue
        self.notificationQueue = notificationQueue
    }

    func startMonitoringAirgap(_ update: @escaping (Bool, Bool) -> Void) {
        adaptee.pathUpdateHandler = { [weak self] path in
            guard let self else { return }
            let isWifiOn: Bool = path.usesInterfaceType(.wifi)
            var currentInterfaces = path.availableInterfaces
            currentInterfaces.removeAll(where: { $0.type == .wifi })
            let isAirplaneModeOn = currentInterfaces.isEmpty
            notificationQueue.async {
                update(isAirplaneModeOn, isWifiOn)
            }
        }
        adaptee.start(queue: monitoringQueue)
    }
}

final class AirgapMediatingStub: AirgapMediating {
    func startMonitoringAirgap(_ update: @escaping (Bool, Bool) -> Void) {
        update(true, false)
    }
}
