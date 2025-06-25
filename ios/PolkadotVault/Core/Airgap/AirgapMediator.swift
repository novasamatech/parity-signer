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
}

final class AirgapMediator: AirgapMediating {
    private let adaptee: PathMonitorProtocol
    private let monitoringQueue: DispatchQueue
    private let notificationQueue: DispatchQueue
    private let notificationCenter: NotificationCenter
    private let locationManager: LocationServicesManaging.Type
    private var wifiSubject = CurrentValueSubject<Bool, Never>(false)
    private var airplaneSubject = CurrentValueSubject<Bool, Never>(true)
    private var locationSubject = CurrentValueSubject<Bool, Never>(false)

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
        notificationCenter: NotificationCenter = NotificationCenter.default,
        monitoringQueue: DispatchQueue = DispatchQueue.global(qos: .userInteractive),
        notificationQueue: DispatchQueue = DispatchQueue.main
    ) {
        self.adaptee = adaptee
        self.locationManager = locationManager
        self.monitoringQueue = monitoringQueue
        self.notificationCenter = notificationCenter
        self.notificationQueue = notificationQueue

        listenToLocationChanges()
        startMonitoringAirgap()
    }

    private func startMonitoringAirgap() {
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

    private func stopMonitoringAirgap() {
        adaptee.cancel()
        adaptee.pathUpdateHandler = nil
    }

    private func listenToLocationChanges() {
        notificationCenter.addObserver(
            forName: UIApplication.didBecomeActiveNotification,
            object: nil,
            queue: nil
        ) { [weak self] _ in
            guard let self else { return }
            updateLocationServicesStatus()
        }
        updateLocationServicesStatus()
    }

    private func stopLocationStatusListening() {
        notificationCenter.removeObserver(self)
    }

    private func updateLocationServicesStatus() {
        monitoringQueue.async {
            let isEnabled = self.locationManager.locationServicesEnabled()
            self.notificationQueue.async {
                self.locationSubject.send(isEnabled)
            }
        }
    }

    deinit {
        stopMonitoringAirgap()
        stopLocationStatusListening()
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
}
