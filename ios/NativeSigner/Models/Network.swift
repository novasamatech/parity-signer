//
//  Network.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import SwiftUI

struct Network: Codable {
    var key: String
    var color: String
    var logo: String
    var order: String
    var secondaryColor: String
    var title: String
}

extension Network {
    static var networkData: [Network] {
        [
            Network(key: "111111", color: "0xFFFFFF", logo: "1", order: "0", secondaryColor: "0xFFFFFF", title: "Ololonet"),
            Network(key: "111111", color: "0xFFFFFF", logo: "2", order: "1", secondaryColor: "0xFFFFFF", title: "Pyshpysh"),
            Network(key: "111111", color: "0xFFFFFF", logo: "3", order: "2", secondaryColor: "0xFFFFFF", title: "Kekeke"),
            Network(key: "111111", color: "0xFFFFFF", logo: "4", order: "3", secondaryColor: "0xFFFFFF", title: "Kuskuskus")
        ]
    }
}

//MARK: network management

extension SignerDataModel {
    func refreshNetworks() {
        let res = get_all_networks_for_network_selector(self.err_ptr, self.dbName)
        print("refresh call")
        if self.err_ptr.pointee.code == 0 {
            if let networksJSON = String(cString: res!).data(using: .utf8) {
            print(networksJSON)
                guard let networks = try? JSONDecoder().decode([Network].self, from: networksJSON) else {
                    print("JSON decoder failed on networks")
                    print(networksJSON)
                    signer_destroy_string(res!)
                    return
                }
                self.networks = networks
            } else {
                    print("networksJSON corrupted")
                    print(String(cString: res!))
            }
            signer_destroy_string(res!)
        } else {
            self.handleRustError()
        }
    }
    
    func selectNetwork(network: Network) {
        self.selectedNetwork = network
        self.fetchKeys()
    }
}
