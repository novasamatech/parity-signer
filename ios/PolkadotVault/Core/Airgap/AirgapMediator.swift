//
//  AirgapMediator.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 15/02/2023.
//

import Combine
import CoreLocation
import Foundation
import Network
import UIKit

struct AirgapState: Equatable {
    let isAirplaneModeOn: Bool
    let isWifiOn: Bool
    let isLocationServiceEnabled: Bool
}

// sourcery: AutoMockable
protocol LocationServicesManaging: AnyObject {
    static func locationServicesEnabled() -> Bool
}

extension CLLocationManager: LocationServicesManaging {}

protocol AirgapMediating: AnyObject {
    var isConnectedPublisher: AnyPublisher<Bool, Never> { get }
    var airgapPublisher: AnyPublisher<AirgapState, Never> { get }

    func startMonitoringAirgap()
}

final class AirgapMediator: AirgapMediating {
    private let adaptee: PathMonitorProtocol
    private let monitoringQueue: DispatchQueue
    private let notificationQueue: DispatchQueue
    private let locationManager: LocationServicesManaging.Type
    private var wifiSubject = CurrentValueSubject<Bool, Never>(true)
    private var airplaneSubject = CurrentValueSubject<Bool, Never>(false)
    private var locationSubject = CurrentValueSubject<Bool, Never>(true)

    var airgapPublisher: AnyPublisher<AirgapState, Never> {
        Publishers.CombineLatest3(wifiSubject, airplaneSubject, locationSubject)
            .map { wifi, airplane, locationServiceEnabled in
                AirgapState(
                    isAirplaneModeOn: airplane,
                    isWifiOn: wifi,
                    isLocationServiceEnabled: locationServiceEnabled
                )
            }
            .removeDuplicates()
            .eraseToAnyPublisher()
    }

    var isConnectedPublisher: AnyPublisher<Bool, Never> {
        airgapPublisher
            .map { airgapState in
                airgapState.isLocationServiceEnabled || airgapState.isWifiOn || !airgapState.isAirplaneModeOn
            }
            .removeDuplicates()
            .eraseToAnyPublisher()
    }

    init(
        adaptee: PathMonitorProtocol = NWPathMonitor(),
        locationManager: LocationServicesManaging.Type = CLLocationManager.self,
        monitoringQueue: DispatchQueue = DispatchQueue.global(qos: .userInteractive),
        notificationQueue: DispatchQueue = DispatchQueue.main
    ) {
        self.adaptee = adaptee
        self.locationManager = locationManager
        self.monitoringQueue = monitoringQueue
        self.notificationQueue = notificationQueue
        listenToLocationChanges()
    }

    func startMonitoringAirgap() {
        adaptee.pathUpdateHandler = { [weak self] path in
            guard let self else { return }
            let isWifiOn: Bool = path.usesInterfaceType(.wifi)
            var currentInterfaces = path.availableInterfaces
            currentInterfaces.removeAll(where: { $0.type == .wifi })
            notificationQueue.async {
                self.airplaneSubject.send(currentInterfaces.isEmpty)
                self.wifiSubject.send(isWifiOn)
            }
        }
        adaptee.start(queue: monitoringQueue)
    }

    private func listenToLocationChanges() {
        NotificationCenter.default.addObserver(
            forName: UIApplication.didBecomeActiveNotification,
            object: nil,
            queue: nil
        ) { [weak self] _ in
            guard let self else { return }
            updateLocationServicesStatus()
        }
        updateLocationServicesStatus()
    }

    deinit {
        NotificationCenter.default.removeObserver(self)
    }

    private func updateLocationServicesStatus() {
        monitoringQueue.async {
            let isEnabled = self.locationManager.locationServicesEnabled()
            self.notificationQueue.async {
                self.locationSubject.send(isEnabled)
            }
        }
    }
}

final class AirgapMediatingStub: AirgapMediating {
    var stubState = AirgapState(isAirplaneModeOn: true, isWifiOn: false, isLocationServiceEnabled: false)
    var stubIsConnected = false

    var isConnectedPublisher: AnyPublisher<Bool, Never> {
        Just(stubIsConnected)
            .eraseToAnyPublisher()
    }

    var airgapPublisher: AnyPublisher<AirgapState, Never> {
        Just(stubState)
            .eraseToAnyPublisher()
    }

    func startMonitoringAirgap() {}
}
