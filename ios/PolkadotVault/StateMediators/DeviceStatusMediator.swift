import Foundation

protocol DeviceStatusMediating {
    func deviceBecameOnline()
    func deviceWentOffline()
}

// Class is designed to track and log once device became online
final class DeviceStatusMediator {
    enum State {
        case notReported
        case reported
        case reporting

        var alredyReported: Bool {
            switch self {
            case .notReported:
                false
            case .reporting,
                 .reported:
                true
            }
        }
    }

    private let backendService: BackendService
    private var state: State = .notReported

    init(backendService: BackendService = BackendService()) {
        self.backendService = backendService
    }
}

extension DeviceStatusMediator: DeviceStatusMediating {
    func deviceBecameOnline() {
        guard !state.alredyReported else {
            return
        }

        backendService.performCall {
            try historyDeviceWasOnline()
        } completion: { [weak self] (result: Result<Void, ErrorDisplayed>) in
            switch result {
            case .success:
                self?.state = .reported
            case .failure:
                self?.state = .notReported
            }
        }
    }

    func deviceWentOffline() {
        state = .notReported
    }
}
