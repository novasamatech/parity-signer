//
//  MDeriveKey.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 13.12.2021.
//

import Foundation

struct MDeriveKey: Decodable, Equatable {
    var seed_name: String
    var network_title: String
    var network_logo: String
    var network_specs_key: String
    var suggested_derivation: String
    var keyboard: Bool
    var derivation_check: DerivationCheck?
}

struct onlyDerivationCheck: Decodable {
    var derivation_check: DerivationCheck?
}

extension MDeriveKey {
    /**
     * Call this on every derivation path update
     */
    func updateDerivationCheck(path: String, dbName: String) -> DerivationCheck? {
        let res = path_check(nil, seed_name, path, network_specs_key, dbName)
        print(String(cString: res!))
        if let derivationCheckJSON = String(cString: res!).data(using: .utf8) {
            if let newDerivationCheck = try? JSONDecoder().decode(onlyDerivationCheck.self, from: derivationCheckJSON) {
                return newDerivationCheck.derivation_check
            } else {
                print("check path JSON format error")
                signer_destroy_string(res!)
                return nil
            }
        } else {
            print("check path string error")
            signer_destroy_string(res!)
            return nil
        }
    }
}
