//
//  Camera.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 20.7.2021.
//

import Foundation
import Combine
import AVFoundation

final class CameraViewModel: ObservableObject {
    private let service = CameraService()
    
    @Published var payload: String?
    @Published var captured: Int?
    @Published var total: Int?
    @Published var progress = 0.0
    
    @Published var showAlertError = false
    
    var isFlashOn = false
    
    var session: AVCaptureSession
    
    private var subscriptions = Set<AnyCancellable>()
    
    init() {
        self.session = service.session
        
        service.$payload.sink { [weak self] (payload) in
            guard let value = payload else {return}
            self?.payload = value
        }
        .store(in: &self.subscriptions)
        
        service.$captured.sink { [weak self] (captured) in
            guard let value = captured else {return}
            self?.captured = value
        }
        .store(in: &self.subscriptions)
        
        service.$total.sink { [weak self] (total) in
            guard let value = total else {return}
            self?.total = value
        }
        .store(in: &self.subscriptions)
    }
    
    func configure() {
        service.checkForPermissions()
        service.configure()
    }
    
    func shutdown() {
        print(self.payload ?? "Nothing")
        service.stop()
    }
}
