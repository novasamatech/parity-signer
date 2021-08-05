//
//  Canary.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 5.8.2021.
//

import Foundation
import Network

class Canary: ObservableObject {
    @Published var dead: Bool = false
    let monitor = NWPathMonitor()
    let queue = DispatchQueue.global(qos: .background)
    
    init() {
        self.monitor.pathUpdateHandler = {path in
            if path.availableInterfaces.count == 0 {
                DispatchQueue.main.async {
                    self.dead = false
                }
            } else {
                DispatchQueue.main.async {
                    self.dead = true
                }
            }
        }
        
        monitor.start(queue: self.queue)
    }
}
